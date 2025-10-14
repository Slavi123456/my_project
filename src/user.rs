use std::{fmt::Display, sync::Arc};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::MySqlPool;
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

    pub fn get_user_profile(&self) -> UserProfile{
        self.base.get_user_profile()
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
        self.email == login.email && self.password == login.password
    }

    pub fn get_user_profile(&self) -> UserProfile {
        UserProfile { first_name: self.first_name.clone(), last_name: self.last_name.clone(), email: self.email.clone() }
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

#[derive(Serialize)]
pub struct UserProfile {
    first_name: String,
    last_name: String,
    email: String,
}
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
    db: Option<MySqlPool>,
}

impl AppState {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        let db_pool = MySqlPool::connect(db_url).await;

        match db_pool {
            Ok(db) => Ok(Self {
                users: Arc::new(Mutex::new(Vec::new())),
                sessions: Arc::new(Mutex::new(Vec::new())),
                db: Some(db),
            }),
            Err(error) => Err(error),
        }
    }
    pub async fn new_without_db() -> Result<Self, sqlx::Error> {
        Ok(Self {
                users: Arc::new(Mutex::new(Vec::new())),
                sessions: Arc::new(Mutex::new(Vec::new())),
                db: None,
            })
    }
    pub async fn add_user(&self, user: User) -> Result<(), sqlx::Error> {
        println!("->> HANDLER - add_user");

        if let Err(err) = user.validate() {
            println!("->> ERROR - cannot add user {}", err);
        }

        let mut users = self.users.lock().await;

        //should make a validation for repeated users with one email

        let users_len = users.len();
        let user_for_db = user.clone();
        users.push(StoredUser::new(users_len, user));

        //add user to the data base
        match &self.db {
            Some(pool) => {
                sqlx::query(
            "INSERT INTO users (id, first_name, last_name, email, password) VALUES (?, ?, ?, ?, ?)",
                )
                .bind(&users_len.to_string())
                .bind(&user_for_db.first_name)
                .bind(&user_for_db.last_name)
                .bind(&user_for_db.email)
                .bind(&user_for_db.password)
                .execute(pool)
                .await
                .map_err(|e| {
                    println!("Error inserting user into DB: {}", e);
                    e 
                })?;
            }
            None => {
                println!("->>Error with accessing the db");
            }
        }
        //print the user_count
        self.print_db_user_count().await;

        Ok(())
    }
    pub async fn print_db_user_count(&self) {
        match &self.db {
            Some(pool) => {
                let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
                    .fetch_one(pool)
                    .await
                    .unwrap();

                println!("Total users in db: {}", row.0);
            }
            None => {
                println!("->>Error with accessing the db");
            }
        }
    }
    pub async fn update_user(&self, updated_user: User, target_id: usize) -> Result<(), String> {
        println!("->> HANDLER - update_user");

        if let Err(err) = updated_user.validate() {
            return Err(format!("->> ERROR - cannot update user {}", err));
        }

        let mut users = self.users.lock().await;
        let user_for_db = updated_user.clone();

        //needs something like operator= in c++
        if let Some(user) = users.iter_mut().find(|u| u.id == target_id) {
            user.base.first_name = updated_user.first_name;
            user.base.last_name = updated_user.last_name;
            user.base.email = updated_user.email;
            user.base.password = updated_user.password;

            println!("User updated.");
        } else {
            return Err(format!("->> Error - User not found."));
        }

        //update in the database
        match &self.db {
            Some(pool) => {

                //should fix the unwrap here to someting smarter
                sqlx::query(
                    "UPDATE users SET first_name = ?, last_name = ?, email = ?, password = ? WHERE id = ?"
                )
                .bind(&user_for_db.first_name)
                .bind(&user_for_db.last_name)
                .bind(&user_for_db.email)
                .bind(&user_for_db.password)
                .bind(target_id.to_string())
                .execute(pool)
                .await
                .map_err(|e| {
                    format!("Error updating user into DB: {}", e)
                     
                }).unwrap();
            }
            None => {
                println!("->>Error with accessing the db");
            }
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
    pub async fn get_user_id_from_session (&self, target_session: &str) -> Result<usize, String> {
        let sessions = self.sessions.lock().await;

        let session = sessions.iter().find(|sess| sess.session_id == target_session);
        let target_user_id = match session {
            Some(sess) => sess.user_id,
            None => return Err("Session is invalid".to_string()),
        };

        Ok(target_user_id)
    }
    pub async fn get_user_profile_from_session_id (&self, target_session: &str) -> Result<UserProfile, String>{
        let sessions = self.sessions.lock().await;

        let session = sessions.iter().find(|sess| sess.session_id == target_session);
        
        //I will do it here again to avoid dead locks
        let target_user_id = match session {
            Some(sess) => sess.user_id,
            None => return Err("Session is invalid".to_string()),
        };

        //Select query from the database later on
        let users = self.users.lock().await;
        let user = users.iter().find(|stored| stored.id == target_user_id);
        let user_profile = match user {
            Some(u) => u.get_user_profile(),
            None => return Err("User not found".to_string()),
        };

        Ok(user_profile)
    }
    ///////////////////////////////////////////////////////////////////////
    pub async fn is_session_valid(&self, target_session: &str) -> bool {
        let sessions = self.sessions.lock().await;
        match sessions.iter().find(|sess| sess.session_id == target_session) {
            Some(_session) => return true,
            None => return false,
        }
    }
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
