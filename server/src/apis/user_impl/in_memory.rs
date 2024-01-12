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

impl<T, S> UserAPI<T, S> for InMemoryUserAPI<T, S>
where
    T: Send + Sync + 'static + Clone + FromBytes<T> + IntoBytes<T>,
    S: Send + Sync + 'static + Clone + FromBytes<S> + IntoBytes<S>,
{
    fn create(&mut self, user: User<T>) {
        self.users.insert(user.username.clone(), user);
    }

    fn read(&mut self, username: &str) -> Option<User<T>> {
        self.users.get(username).cloned()
    }

    fn update(&mut self, name: &String, new_user: User<T>) -> Option<()> {
        if let Some(user) = self.users.get_mut(name) {
            *user = new_user;
            Some(())
        } else {
            None
        }
    }

    fn delete(&mut self, name: &String) -> Option<User<T>> {
        self.users.remove(name)
    }

    fn create_auth_challenge(&mut self, user: &String, c: &S) -> String {
        let uid = Uuid::new_v4().to_string();
        let auth_challenge = AuthChallenge {
            id: uid.clone(),
            user: user.clone(),
            c: c.clone(),
        };
        self.auth_challenges.insert(uid.clone(), auth_challenge);
        uid
    }

    fn delete_auth_challenge(&mut self, id: &String) {
        self.auth_challenges.remove(id);
    }

    fn get_auth_challenge(&mut self, id: &String) -> Option<AuthChallenge<S>> {
        self.auth_challenges.get(id).cloned()
    }
}
