use crate::protocol::{GroupParams, Protocol};
use crate::traits::Random;
use crate::traits::{FromBytes, IntoBytes};
use anyhow::Result;
use num_bigint::{BigUint, RandBigInt};
use num_traits::One;
use rand::rngs::OsRng;

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
        params: &Self::GroupParameters,
        x: &Self::Secret,
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
        params: &Self::GroupParameters,
        k: &Self::CommitmentRandom,
        c: &Self::Challenge,
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
        params: &Self::GroupParameters,
        s: &Self::Response,
        c: &Self::Challenge,
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

impl IntoBytes<BigUint> for BigUint {
    fn to(t: &BigUint) -> Vec<u8> {
        t.to_bytes_be()
    }
}

impl FromBytes<BigUint> for BigUint {
    fn from(bytes: &[u8]) -> Result<BigUint> {
        Ok(BigUint::from_bytes_be(bytes))
    }
}

impl Random<BigUint> for BigUint {
    fn random() -> Result<BigUint> {
        use rand::RngCore;
        let mut rng = OsRng;
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);
        Ok(BigUint::from_bytes_be(&bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::ToBigUint;

    #[test]
    fn biguint_serialization() {
        let original = 34329847u64.to_biguint().unwrap();
        let bytes = BigUint::to(&original);
        let recovered = <BigUint as FromBytes<BigUint>>::from(&bytes).unwrap();
        // ensure reversability of (de)serialization operations
        assert_eq!(original, recovered);
    }
}
