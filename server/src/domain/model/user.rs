use sqlx::{Encode, FromRow};
use uuid::Uuid;

use crate::app_error::error::AppError;
use crate::util::datetime;

#[derive(Clone, Debug, Encode, FromRow)]
pub struct User {
    pub user_id: String,
    pub name: String,
    pub birthday: i32,
    pub email: String,
    pub blood_type: i32,
    pub created_at: i32,
    pub updated_at: i32,
}

impl User {
    pub fn new_create(name: String, birthday: i32, email: String, blood_type: i32) -> Self {
        User {
            user_id: Uuid::new_v4().to_string(),
            name,
            birthday,
            email,
            blood_type,
            created_at: datetime::get_timestamp(),
            updated_at: datetime::get_timestamp(),
        }
    }

    pub fn new_update(user_id: String, name: String) -> User {
        User {
            user_id,
            name,
            birthday: 0,
            blood_type: 0,
            email: String::new(),
            created_at: 0,
            updated_at: datetime::get_timestamp(),
        }
    }

    pub fn new_user_name(name: String) -> Result<String, AppError> {
        if name.is_empty() {
            return Err(AppError::BadRequest("user name is empty".to_string()));
        }
        Ok(name)
    }

    pub fn new_user_birthday(birthday: i32) -> Result<i32, AppError> {
        if birthday < 0 {
            return Err(AppError::BadRequest("invalid user birthday".to_string()));
        }

        Ok(birthday)
    }

    pub fn new_user_email(email: String) -> Result<String, AppError> {
        if email.is_empty() {
            return Err(AppError::BadRequest("user email is empty".to_string()));
        }
        Ok(email)
    }
}
