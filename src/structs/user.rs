use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::structs::{login::LoginInfo, traits::Extractable};

struct UserConsts {}
impl UserConsts {
    const MIN_NAME_LENGHT: usize = 2;
    const MIN_PASSWORD_LENGHT: usize = 8;
}
////////////////////////////////////////////////////////////////////
pub struct StoredUser {
    id: usize,
    base: User,
}

impl StoredUser {
    pub fn new(id: usize, base: User) -> Result<Self, String> {
        Ok(Self {
            id: id,
            base: base.clone(),
        })
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
    ////////////////////////////////////////////////
    //Constructors
    pub fn new(
        first_name: String,
        last_name: String,
        email: String,
        password: String,
    ) -> Result<Self, String> {
        let user = User {
            first_name,
            last_name,
            email,
            password,
        };
        user.validate()?;
        Ok(user)
    }
    pub fn copy_operator(&mut self, other: &User) -> Result<(), String> {
        self.set_email(other.email.clone())?;
        self.set_first_name(other.first_name.clone())?;
        self.set_last_name(other.last_name.clone())?;
        self.set_password(other.password.clone())?;
        Ok(())
    }

    ////////////////////////////////////////////////
    //Getters and Setters
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
    pub fn set_email(&mut self, new_email: String) -> Result<(), String> {
        if User::validate_email(&new_email) {
            return Err(String::from("Invalid email"));
        }
        self.email = new_email;
        Ok(())
    }
    pub fn set_first_name(&mut self, new_first_name: String) -> Result<(), String> {
        if User::validate_name(&new_first_name) {
            return Err(String::from("Invalid email"));
        }
        self.first_name = new_first_name;
        Ok(())
    }
    pub fn set_last_name(&mut self, new_last_name: String) -> Result<(), String> {
        if User::validate_name(&new_last_name) {
            return Err(String::from("Invalid email"));
        }
        self.last_name = new_last_name;
        Ok(())
    }
    pub fn set_password(&mut self, new_password: String) -> Result<(), String> {
        if User::validate_password(&new_password) {
            return Err(String::from("Invalid email"));
        }
        self.password = new_password;
        Ok(())
    }

    ////////////////////////////////////////////////

    pub fn validate(&self) -> Result<(), String> {
        //Simple validation

        if !User::validate_name(&self.first_name) {
            println!("First Name is {}", self.first_name);
            return Err(format!(
                "First name must be at least {} characters long.",
                UserConsts::MIN_NAME_LENGHT
            ));
        }

        if !User::validate_name(&self.last_name) {
            println!("Last Name is {}", self.last_name);

            return Err(format!(
                "Last name must be at least {} characters long.",
                UserConsts::MIN_NAME_LENGHT
            ));
        }

        //should make a validation for repeated users with one email
        if !User::validate_email(&self.email) {
            println!("Email is {}", self.email);
            return Err(format!("Email must be valid (contain @ and .)"));
        }

        if !User::validate_password(&self.password) {
            println!("Password is {}", self.password);
            return Err(format!(
                "Password must be at least {} characters long.",
                UserConsts::MIN_PASSWORD_LENGHT
            ));
        }

        Ok(())
    }

    pub fn validate_email(new_email: &str) -> bool {
        !new_email.is_empty() && new_email.contains('@') && new_email.contains('.')
    }
    pub fn validate_name(new_name: &str) -> bool {
        !new_name.is_empty() && new_name.len() >= UserConsts::MIN_NAME_LENGHT
    }
    pub fn validate_password(new_password: &str) -> bool {
        !new_password.is_empty() && new_password.len() >= UserConsts::MIN_PASSWORD_LENGHT
    }

    ////////////////////////////////////////////////
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

impl UserProfile {
    pub fn first_name(&self) -> &str {
        &self.first_name
    }
    pub fn last_name(&self) -> &str {
        &self.last_name
    }
    pub fn email(&self) -> &str {
        &self.email
    }
}
