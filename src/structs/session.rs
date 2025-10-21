use std::fmt::Display;

#[derive(Clone)]
pub struct Session {
    session_id: String,
    user_id: usize,
}
impl Session {
    pub fn new(session: String, user_id: usize) -> Result<Self, String> {
        if session.is_empty() {
            return Err(String::from("Invalid email"));
        }
        Ok(Self {
            session_id: session.clone(),
            user_id: user_id,
        })
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }
    pub fn user_id(&self) -> &usize {
        &self.user_id
    }
}

impl Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Session id {} user id {}", self.session_id, self.user_id)
    }
}
