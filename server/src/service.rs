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
