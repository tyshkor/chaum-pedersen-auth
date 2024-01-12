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
