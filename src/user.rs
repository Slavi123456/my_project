use std::{fmt::Display, sync::Arc};

use serde::{Deserialize, de::DeserializeOwned};
use tokio::sync::Mutex;
////////////////////////////////////////////////////////////////////
pub trait Extractable: DeserializeOwned + Sized {}
////////////////////////////////////////////////////////////////////
#[derive(Deserialize)]
pub struct LoginInfo {
    email: String,
    password: String,
}

impl Extractable for LoginInfo {}
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

    pub fn match_credentials(&self, login: &LoginInfo) -> bool {
        self.email == login.email && self.password == login.password
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

#[derive(Clone)]
pub struct Session {
    session_id: String,
    user_id: usize,
}
impl Session {
    pub fn new(session: String, user_id: usize) -> Self {
        Self {
            session_id: session,
            user_id: user_id,
        }
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }
}

impl Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Session id {} user id {}", self.session_id, self.user_id)
    }
}

////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub struct AppState {
    users: Arc<Mutex<Vec<StoredUser>>>,
    sessions: Arc<Mutex<Vec<Session>>>,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(Vec::new())),
            sessions: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub async fn add_user(&self, user: User) {
        println!("->> HANDLER - add_user");

        if let Err(err) = user.validate() {
            println!("->> ERROR - cannot add user {}", err);
        }

        let mut users = self.users.lock().await;

        //should make a validation for repeated users with one email

        let users_len = users.len();
        users.push(StoredUser::new(users_len, user));
    }

    pub async fn update_user(&self, updated_user: User, target_id: usize) -> Result<(), String> {
        println!("->> HANDLER - update_user");

        if let Err(err) = updated_user.validate() {
            return Err(format!("->> ERROR - cannot update user {}", err));
        }

        let mut users = self.users.lock().await;

        //needs something like operator= in c++
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

    pub async fn find_user(&self, login: LoginInfo) -> Result<usize, String> {
        println!("->> HANDLER - find_user");

        let users = self.users.lock().await;

        if let Some(user) = users.iter().find(|u| u.base.match_credentials(&login)) {
            println!("User {} is valid.", user);
            return Ok(user.id);
        } else {
            return Err(format!("->> Error - User not found."));
        }
    }

    ///////////////////////////////////////////////////////////////////////
    pub async fn add_session(&self, user_id: usize) -> Result<Session, String> {
        println!("->> HANDLER - add_session");

        let mut sessions = self.sessions.lock().await;
        let sessions_len = sessions.len();

        let new_session = Session::new(sessions_len.to_string(), user_id);
        sessions.push(new_session.clone());
        Ok(new_session)
    }

    pub async fn print_sessions(&self) {
        println!("->> HANDLER - print_sessions");
        let sessions = self.sessions.lock().await;
        for ses in sessions.iter() {
            println!("{}", ses);
        }
    }

    pub async fn delete_session(&mut self, target_session: &str) {
        println!("->> HANDLER - delete_session");
        let mut sessions = self.sessions.lock().await;

        sessions.retain(|s| s.session_id != target_session);
    }
}
