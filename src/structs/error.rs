use std::fmt::Display;

pub enum AppError {
    SqlxError(sqlx::Error),
    UserError(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::SqlxError(e) => write!(f, "Database error: {}", e),
            AppError::UserError(msg) => write!(f, "User error: {}", msg),
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::SqlxError(value)
    }
}
