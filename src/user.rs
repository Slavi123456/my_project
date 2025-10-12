use std::fmt::Display;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    username: String,
    password: String,
}

impl User {}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User {} password {}", self.username, self.password)
    }
}
