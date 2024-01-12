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
