use crate::apis::user_impl::in_memory::InMemoryUserAPI;
use crate::apis::{user::UserAPI, user::User};
use chaum_pedersen::traits::{FromBytes, IntoBytes};
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use chaum_pedersen::protocol::{Protocol, GroupParams};

// Protobuf generated module
pub mod zkp_auth {
    tonic::include_proto!("zkp_auth");
}

// Protobuf imports
use zkp_auth::{
    auth_server::Auth, AuthenticationAnswerRequest, AuthenticationAnswerResponse,
    AuthenticationChallengeRequest, AuthenticationChallengeResponse, RegisterRequest,
    RegisterResponse,
};

/// A struct representing the authentication service.
pub struct AuthService<C, T, S> {
    params: GroupParams<T>,
    api: Mutex<Box<dyn UserAPI<T, S> + Send + Sync>>,
    _type_phantom: std::marker::PhantomData<C>,
    _scalar_phantom: std::marker::PhantomData<S>,
}

impl<
        C,
        T: Send + Sync + Clone + FromBytes<T> + IntoBytes<T> + 'static,
        S: Send + Sync + Clone + FromBytes<S> + IntoBytes<S> + 'static,
    > AuthService<C, T, S>
{
    pub fn new(params: GroupParams<T>) -> Self {
        let api = Mutex::new(
            Box::new(InMemoryUserAPI::<T, S>::new()) as Box<dyn UserAPI<T, S> + Send + Sync>
        );
        Self {
            params,
            api,
            _type_phantom: std::marker::PhantomData,
            _scalar_phantom: std::marker::PhantomData,
        }
    }
}

#[tonic::async_trait]
impl<C, T, S> Auth for AuthService<C, T, S>
where
    T: Send + Sync + 'static + Clone + FromBytes<T> + IntoBytes<T>,
    S: Send + Sync + 'static + Clone + FromBytes<S> + IntoBytes<S>,
    C: Protocol<
            Response = S,
            CommitmentRandom = S,
            Challenge = S,
            Secret = S,
            GroupParameters = GroupParams<T>,
            CommitParameters = (T, T, T, T),
        >
        + 'static
        + std::marker::Sync
        + std::marker::Send,
{
    async fn register(
        &self, request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let req = request.into_inner();

        let y1 =
            T::from(&req.y1).or_else(|_| Err(Status::invalid_argument("Invalid y1")))?;
        let y2 =
            T::from(&req.y2).or_else(|_| Err(Status::invalid_argument("Invalid y2")))?;

        let user = User {
            username: req.user.clone(),
            y1,
            y2,
            r1: None,
            r2: None,
        };

        let mut api = self.api.lock().await;
        api.create(user);

        let reply = RegisterResponse {};
        Ok(Response::new(reply))
    }

    async fn create_authentication_challenge(
        &self, request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        let req = request.into_inner();
        let challenge = C::challenge(&self.params);

        let user = {
            let mut api = self.api.lock().await;
            let mut user = api
                .read(&req.user)
                .ok_or_else(|| Status::not_found("User not found"))?;
            user.r1 = Some(
                T::from(&req.r1)
                    .or_else(|_| Err(Status::invalid_argument("Invalid r1")))?,
            );
            user.r2 = Some(
                T::from(&req.r2)
                    .or_else(|_| Err(Status::invalid_argument("Invalid r2")))?,
            );
            user.clone()
        };

        let auth_id = {
            let mut api = self.api.lock().await;
            api.update(&user.username, user.clone());
            api.create_auth_challenge(&req.user, &challenge)
        };

        let reply = AuthenticationChallengeResponse {
            auth_id,
            c: S::to(&challenge),
        };
        Ok(Response::new(reply))
    }

    async fn verify_authentication(
        &self, request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        let req = request.into_inner();

        let challenge = {
            let mut api = self.api.lock().await;
            api.get_auth_challenge(&req.auth_id)
                .ok_or_else(|| Status::not_found("Challenge not found"))?
        };

        let user = {
            let mut api = self.api.lock().await;
            api.read(&challenge.user)
                .ok_or_else(|| Status::not_found("User not found"))?
        };

        let s = S::from(&req.s).or_else(|_| Err(Status::invalid_argument("Invalid s")))?;
        let params = self.params.clone();
        let verified = C::verify(
            &params,
            &s,
            &challenge.c,
            &(user.y1, user.y2, user.r1.unwrap(), user.r2.unwrap()),
        );

        if !verified {
            return Err(Status::invalid_argument("Invalid authentication"));
        }
        let session_id = Uuid::new_v4().to_string();
        let reply = AuthenticationAnswerResponse { session_id };

        let mut api = self.api.lock().await;
        api.delete_auth_challenge(&req.auth_id);
        Ok(Response::new(reply))
    }
}
