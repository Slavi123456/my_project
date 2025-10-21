use serde::Deserialize;

use crate::structs::{
    traits::Extractable,
    user::{validate_email, validate_password},
};

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct LoginInfo {
    email: String,
    password: String,
}
impl LoginInfo {
    pub fn new(email: &str, password: &str) -> Result<Self, String> {
        if !validate_email(email) || !validate_password(password) {
            return Err(String::from("Couldn't create LoginInfo"));
        }

        Ok(Self {
            email: email.to_string(),
            password: password.to_string(),
        })
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}

impl Extractable for LoginInfo {}
