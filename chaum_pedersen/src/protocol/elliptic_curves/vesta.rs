use crate::protocol::{Protocol, GroupParams};
use crate::traits::{IntoBytes, FromBytes};
use crate::traits::Random;
use pasta_curves::group::ff::Field;
use pasta_curves::group::ff::{FromUniformBytes, PrimeField};
use pasta_curves::group::Group;
use pasta_curves::group::GroupEncoding;
use pasta_curves::vesta::Point;
use pasta_curves::vesta::Scalar;
use pasta_curves::Ep;
use pasta_curves::Fp;
use rand_core::OsRng;
use anyhow::Result;

use super::errors::EllipticCurveError;

pub struct VestaEllipticCurve {}

impl Protocol for VestaEllipticCurve {
    type Secret = Scalar;
    type Response = Scalar;
    type Challenge = Scalar;
    type CommitmentRandom = Scalar;
    type GroupParameters = GroupParams<Point>;
    type CommitParameters = (Point, Point, Point, Point);

    fn commitment(
        params: &Self::GroupParameters, x: &Self::Secret,
    ) -> (Self::CommitParameters, Self::CommitmentRandom)
    where
        Self: Sized,
    {
        let y1 = params.g * <Scalar as From<Scalar>>::from(x.clone());
        let y2 = params.h * <Scalar as From<Scalar>>::from(x.clone());
        let mut rng = OsRng;
        let k = <Scalar as Field>::random(&mut rng);
        let r1 = params.g * k;
        let r2 = params.h * k;
        ((y1, y2, r1, r2), k)
    }

    fn challenge(_: &GroupParams<Point>) -> Self::Challenge {
        let mut rng = OsRng;
        <Scalar as Field>::random(&mut rng)
    }

    fn challenge_response(
        _: &Self::GroupParameters, k: &Self::CommitmentRandom, c: &Self::Challenge,
        x: &Self::Secret,
    ) -> Self::Response
    where
        Self: Sized,
    {
        k + (c * x)
    }

    fn verify(
        params: &Self::GroupParameters, s: &Self::Response, c: &Self::Challenge,
        cp: &Self::CommitParameters,
    ) -> bool {
        let (y1, y2, r1, r2) = cp;
        (params.g * s == r1 + (y1 * c)) && (params.h * s == r2 + (y2 * c))
    }
}

impl IntoBytes<Point> for Point {
    fn to(t: &Point) -> Vec<u8> {
        t.to_bytes().to_vec()
    }
}

impl FromBytes<Point> for Point {
    fn from(bytes: &[u8]) -> Result<Point> {
        let array: [u8; 32] = bytes.try_into().map_err(|_| EllipticCurveError::ScalarInvalidBytesLen)?;

        Ok(Point::from_bytes(&array).unwrap())
    }
}

impl IntoBytes<Scalar> for Scalar {
    fn to(t: &Scalar) -> Vec<u8> {
        t.to_repr().as_slice().to_vec()
    }
}

impl FromBytes<Scalar> for Scalar {
    fn from(bytes: &[u8]) -> Result<Scalar> {
        // pad the array with zeros
        let array = |input: &[u8]| -> [u8; 64] {
            let mut output = [0u8; 64];
            let len = input.len().min(64);
            output[..len].copy_from_slice(&input[..len]);
            output
        };
        Ok(Scalar::from_uniform_bytes(&array(bytes)))
    }
}

impl Random<Ep> for Ep {
    fn random() -> Result<Ep> {
        Ok(<Ep as Group>::random(&mut OsRng))
    }
}

impl Random<Fp> for Fp {
    fn random() -> Result<Fp> {
        Ok(<Fp as Field>::random(&mut OsRng))
    }
}

