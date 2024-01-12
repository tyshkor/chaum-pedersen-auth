use crate::apis::user::UserAPI;
use crate::apis::user::User;
use chaum_pedersen::traits::{FromBytes, IntoBytes};
use std::collections::HashMap;
use uuid::Uuid;

use crate::apis::user::AuthChallenge;

/// This struct for an in-memory implementation of the `UserAPI` trait using hash maps.
pub struct InMemoryUserAPI<T, S> {
    users: HashMap<String, User<T>>,
    auth_challenges: HashMap<String, AuthChallenge<S>>,
}

impl<T, S> InMemoryUserAPI<T, S> {
    pub fn new() -> Self {
        InMemoryUserAPI {
            users: HashMap::new(),
            auth_challenges: HashMap::new(),
        }
    }
}
