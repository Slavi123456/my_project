use serde::Deserialize;

use crate::structs::traits::Extractable;

#[derive(Deserialize)]
pub struct LoginInfo {
    email: String,
    password: String,
}
impl LoginInfo {
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}

impl Extractable for LoginInfo {}
