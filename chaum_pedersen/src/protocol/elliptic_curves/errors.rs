use thiserror::Error;

#[derive(Debug, Error)]
pub enum EllipticCurveError {
    #[error("Invalid bytes length for Scalar")]
    ScalarInvalidBytesLen,
}
