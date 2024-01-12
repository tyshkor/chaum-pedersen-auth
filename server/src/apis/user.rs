#[derive(Debug, Clone)]
pub struct User<T> {
    pub username: String,
    pub y1: T,
    pub y2: T,
    pub r1: Option<T>,
    pub r2: Option<T>,
}

#[derive(Debug, Clone)]
pub struct AuthChallenge<S> {
    pub id: String,
    pub user: String,
    pub c: S,
}

/// Trait defining the API for User struct.
///
/// This trait abstracts the CRUD (Create, Read, Update, Delete) operations
/// and authentication challenge related operations for user data.
pub trait UserAPI<T, S> {
    fn create(&mut self, user: User<T>);

    fn read(&mut self, username: &str) -> Option<User<T>>;

    fn update(&mut self, name: &String, user: User<T>) -> Option<()>;

    fn delete(&mut self, name: &String) -> Option<User<T>>;

    fn create_auth_challenge(&mut self, user: &String, c: &S) -> String;

    fn delete_auth_challenge(&mut self, id: &String);

    fn get_auth_challenge(&mut self, id: &String) -> Option<AuthChallenge<S>>;
}
