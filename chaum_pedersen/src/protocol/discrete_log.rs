use crate::protocol::{Protocol, GroupParams};
use crate::traits::{IntoBytes, FromBytes};
use crate::traits::Random;
use num_bigint::{BigUint, RandBigInt};
use num_traits::One;
use rand::rngs::OsRng;
use anyhow::Result;

#[derive(Clone)]
pub struct DiscreteLog {}

