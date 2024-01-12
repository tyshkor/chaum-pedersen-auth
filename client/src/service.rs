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
