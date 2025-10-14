use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::structs::{login::LoginInfo, traits::Extractable};

////////////////////////////////////////////////////////////////////
pub struct StoredUser {
    id: usize,
    base: User,
}

impl StoredUser {
    pub fn new(id: usize, base: User) -> Self {
        Self {
            id: id,
            base: User::copy(base),
        }
    }

    pub fn get_user_profile(&self) -> UserProfile {
        self.base.get_user_profile()
    }

    pub fn user_id(&self) -> usize {
        self.id
    }

    pub fn get_base(&self) -> &User {
        &self.base
    }
    pub fn get_base_mut(&mut self) -> &mut User {
        &mut self.base
    }
}

impl Display for StoredUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id: {} {}", self.id, self.base)
    }
}
////////////////////////////////////////////////////////////////////
#[derive(Clone, Deserialize)]
pub struct User {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

impl User {
    // pub fn new(first_name: String, last_name: String, email: String, password: String) -> Self {
    //     Self {
    //         first_name,
    //         last_name,
    //         email,
    //         password,
    //     }
    // }
    pub fn copy_operator(&mut self, other: &User) {
        self.set_email(other.email.clone());
        self.set_first_name(other.first_name.clone());
        self.set_last_name(other.last_name.clone());
        self.set_password(other.password.clone());
    }

    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn first_name(&self) -> &str {
        &self.first_name
    }
    pub fn last_name(&self) -> &str {
        &self.last_name
    }
    pub fn password(&self) -> &str {
        &self.password
    }
    pub fn set_email(&mut self, new_email: String) {
        self.email = new_email;
    }
    pub fn set_first_name(&mut self, new_first_name: String) {
        self.password = new_first_name;
    }
    pub fn set_last_name(&mut self, new_last_name: String) {
        self.password = new_last_name;
    }
    pub fn set_password(&mut self, new_password: String) {
        self.password = new_password;
    }

    pub fn copy(user: User) -> Self {
        Self {
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            password: user.password,
        }
    }
    pub fn validate(&self) -> Result<(), String> {
        //Simple validation

        if self.first_name.trim().len() < 2 {
            println!("First Name is {}", self.first_name);
            return Err("First name must be at least 2 characters long.".into());
        }

        if self.last_name.trim().len() < 2 {
            println!("Last Name is {}", self.last_name);
            return Err("Last name must be at least 2 characters long.".into());
        }

        if !self.email.contains('@') || !self.email.contains('.') {
            println!("Email is {}", self.email);
            return Err("Email must be valid (contain @ and .)".into());
        }

        if self.password.len() < 8 {
            println!("Password is {}", self.password);
            return Err("Password must be at least 8 characters long.".into());
        }

        Ok(())
    }

    pub fn match_credentials(&self, login: &LoginInfo) -> bool {
        self.email == login.email() && self.password == login.password()
    }

    pub fn get_user_profile(&self) -> UserProfile {
        UserProfile {
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
        }
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User {} {} email {} password {}",
            self.first_name, self.last_name, self.email, self.password
        )
    }
}

impl Extractable for User {}

////////////////////////////////////////////////////////////////////
#[derive(Serialize)]
pub struct UserProfile {
    first_name: String,
    last_name: String,
    email: String,
}
