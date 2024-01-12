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
