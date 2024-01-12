use anyhow::Result;
use chaum_pedersen::traits::Random;
use chaum_pedersen::traits::{FromBytes, IntoBytes};
use sha2::{Digest, Sha512};

/// Hashes the provided secret string or generates a random value.
pub fn hash_or_generate_random<T: FromBytes<T> + IntoBytes<T> + Random<T>>(
    secret: Option<&String>,
) -> Result<T> {
    match secret {
        Some(s) => {
            let mut hasher = Sha512::new();
            hasher.update(s);
            let result = hasher.finalize();
            T::from(&result).map_err(|error| error.context("Failed to convert hash to target type"))
        }
        None => T::random().map_err(|error| error.context("Failed to generate random value")),
    }
}
