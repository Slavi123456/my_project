use std::{fmt::Display, sync::Arc};

use serde::Deserialize;
use tokio::sync::Mutex;

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
}

impl Display for StoredUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id: {} {}", self.id, self.base)
    }
}

#[derive(Clone, Deserialize)]
pub struct User {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

impl User {
    pub fn new(first_name: String, last_name: String, email: String, password: String) -> Self {
        Self {
            first_name,
            last_name,
            email,
            password,
        }
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
            return Err("First name must be at least 2 characters long.".into());
        }

        if self.last_name.trim().len() < 2 {
            return Err("Last name must be at least 2 characters long.".into());
        }

        if !self.email.contains('@') || !self.email.contains('.') {
            return Err("Email must be valid (contain @ and .)".into());
        }

        if self.password.len() < 8 {
            return Err("Password must be at least 8 characters long.".into());
        }

        Ok(())
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

#[derive(Clone)]
pub struct AppState {
    users: Arc<Mutex<Vec<StoredUser>>>,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub async fn add_user(&self, user: User) {
        println!("->> HANDLER - add_user");

        if let Err(err) = user.validate() {
            println!("->> ERROR - cannot add user {}", err);
        }

        let mut users = self.users.lock().await;
        let users_len = users.len();
        users.push(StoredUser::new(users_len, user));
    }

    pub async fn update_user(&self, updated_user: User, target_id: usize) -> Result<(), String> {
        println!("->> HANDLER - update_user");

        if let Err(err) = updated_user.validate() {
            return Err(format!("->> ERROR - cannot update user {}", err));
        }

        let mut users = self.users.lock().await;

        if let Some(user) = users.iter_mut().find(|u| u.id == target_id) {
            user.base.first_name = updated_user.first_name;
            user.base.last_name = updated_user.last_name;
            user.base.password = updated_user.password;

            println!("User updated.");
        } else {
            return Err(format!("->> Error - User not found."));
        }

        Ok(())
    }

    // pub async fn get_all_users(&self) -> Vec<User> {
    //     println!("->> HANDLER - get_all_users");
    //     let users = self.users.lock().await;
    //     users.clone()
    // }

    pub async fn print_users(&self) {
        println!("->> HANDLER - print_users");
        let users = self.users.lock().await;
        for user in users.iter() {
            println!("{}", user)
        }
    }
}
