//! # Chaum-Pedersen ZKP Authentication Protocol
//!
//! This crate provides a comprehensive implementation of the Chaum-Pedersen zero-knowledge interactive proof authentication scheme.
//! It is designed to facilitate secure and private authentication processes by leveraging the principles of zero-knowledge proofs.
//! This scheme allows a prover to convince a verifier that they know a secret without revealing the secret itself.

/// Common enums for server and client.
pub mod enums;
/// Implements the Chaum-Pedersen zero-knowledge proof protocol.
pub mod protocol;
/// Functions for type conversions and data formatting.
pub mod traits;
