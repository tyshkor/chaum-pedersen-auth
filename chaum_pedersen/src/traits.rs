//! These traits have to be defined and implemented, as Rust quite reasonably forbids 
//! to implement foreign traits on foreigh types

use anyhow::Result;

/// Trait for converting types to and from byte representations.
pub trait IntoBytes<T> {
    fn to(t: &T) -> Vec<u8>;
}

/// Trait for converting types from byte representations.
/// Similar to `std::convert::From`
pub trait FromBytes<T> {
    fn from(bytes: &[u8]) -> Result<T>
    where
        Self: Sized;
}

/// Trait for generating random values of a given type.
/// /// Similar to `std::convert::Into`
pub trait Random<T> {
    fn random() -> Result<T>;
}
