use tonic::codegen::StdError;
use tonic::transport::Channel;
use chaum_pedersen::traits::{FromBytes, IntoBytes};
use chaum_pedersen::protocol::Protocol;
use chaum_pedersen::protocol::GroupParams;
use chaum_pedersen::traits::Random;

/// A module that contains the auto-generated gRPC code for the Zero-Knowledge Proof (ZKP) authentication service.
pub mod zkp_auth {
    tonic::include_proto!("zkp_auth");
}

// Importing specific structures from the `zkp_auth` module.
use zkp_auth::{
    auth_client::AuthClient, AuthenticationAnswerRequest, AuthenticationChallengeRequest,
    RegisterRequest,
};

pub struct AuthClientService {
    /// The gRPC client for the ZKP authentication service.
    client: AuthClient<Channel>,
}

impl AuthClientService {
    pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
    where
        D: std::convert::TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        let client = AuthClient::connect(dst).await?;
        Ok(Self { client })
    }

    pub async fn register(
        &mut self, user: String, y1: Vec<u8>, y2: Vec<u8>,
    ) -> Result<(), tonic::Status> {
        let request = RegisterRequest { user, y1, y2 };
        self.client.register(request).await?;
        Ok(())
    }

    pub async fn create_authentication_challenge(
        &mut self, user: String, r1: Vec<u8>, r2: Vec<u8>,
    ) -> Result<(Vec<u8>, String), tonic::Status> {
        let request = AuthenticationChallengeRequest { user, r1, r2 };
        let response = self.client.create_authentication_challenge(request).await?;
        let inner = response.into_inner();
        Ok((inner.c, inner.auth_id))
    }

    pub async fn verify_authentication(
        &mut self, auth_id: String, s: Vec<u8>,
    ) -> Result<String, tonic::Status> {
        let request = AuthenticationAnswerRequest { auth_id, s };
        let response = self.client.verify_authentication(request).await?;
        Ok(response.into_inner().session_id)
    }
}
