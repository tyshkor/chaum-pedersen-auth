use crate::protocol::{Protocol, GroupParams};
use crate::traits::{IntoBytes, FromBytes};
use crate::traits::Random;
use num_bigint::{BigUint, RandBigInt};
use num_traits::One;
use rand::rngs::OsRng;
use anyhow::Result;

#[derive(Clone)]
pub struct DiscreteLog {}

impl Protocol for DiscreteLog {
    type Secret = BigUint;
    type CommitmentRandom = BigUint;
    type Response = BigUint;
    type Challenge = BigUint;
    type GroupParameters = GroupParams<BigUint>;

    type CommitParameters = (BigUint, BigUint, BigUint, BigUint);

    /// Calculates the commitment for the given secret `x` using the provided group parameters.
    fn commitment(
        params: &Self::GroupParameters, x: &Self::Secret,
    ) -> (Self::CommitParameters, Self::CommitmentRandom)
    where
        Self: Sized,
    {
        let y1 = params.g.modpow(x, &params.p);
        let y2 = params.h.modpow(x, &params.p);
        let mut rng = OsRng;
        let k = rng.gen_biguint_below(&params.p);
        let r1 = params.g.modpow(&k, &params.p);
        let r2 = params.h.modpow(&k, &params.p);
        ((y1, y2, r1, r2), k)
    }

    /// Generates a random challenge for the protocol within the group's range.
    fn challenge(params: &GroupParams<BigUint>) -> BigUint {
        let mut rng = OsRng;
        rng.gen_biguint_below(&params.p)
    }

    /// Generates a random challenge for the protocol within the group's range.
    fn challenge_response(
        params: &Self::GroupParameters, k: &Self::CommitmentRandom, c: &Self::Challenge,
        x: &Self::Secret,
    ) -> Self::Response
    where
        Self: Sized,
    {
        if k >= &(c * x) {
            (k - c * x).modpow(&BigUint::one(), &params.q)
        } else {
            &params.q - (c * x - k).modpow(&BigUint::one(), &params.q)
        }
    }

    /// Verifies the response against the given commitment, challenge, and group.
    fn verify(
        params: &Self::GroupParameters, s: &Self::Response, c: &Self::Challenge,
        cp: &Self::CommitParameters,
    ) -> bool {
        let (y1, y2, r1, r2) = cp;

        let lhs1 = params.g.modpow(s, &params.p);
        let rhs1 = (r1 * y1.modpow(&(&params.p - c - BigUint::one()), &params.p)) % &params.p;
        let lhs2 = params.h.modpow(s, &params.p);
        let rhs2 = (r2 * y2.modpow(&(&params.p - c - BigUint::one()), &params.p)) % &params.p;

        lhs1 == rhs1 && lhs2 == rhs2
    }
}
