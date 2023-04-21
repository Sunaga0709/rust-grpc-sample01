use sqlx::{Encode, FromRow};
use uuid::Uuid;

use crate::app_error::error::AppError;
use crate::util::datetime;

#[derive(Clone, Debug, Encode, FromRow)]
pub struct Todo {
    pub todo_id: String,
    pub user_id: String,
    pub title: String,
    pub description: String,
    pub status: i32,
    pub created_at: i32,
    pub updated_at: i32,
}

impl Todo {
    pub fn new_create(user_id: String, title: String, description: String) -> Self {
        Todo {
            todo_id: Uuid::new_v4().to_string(),
            user_id,
            title,
            description,
            status: 0,
            created_at: datetime::get_timestamp(),
            updated_at: datetime::get_timestamp(),
        }
    }

    pub fn new_update(
        todo_id: String,
        user_id: String,
        title: String,
        description: String,
        status: i32,
    ) -> Self {
        Todo {
            todo_id,
            user_id,
            title,
            description,
            status,
            created_at: 0,
            updated_at: datetime::get_timestamp(),
        }
    }

    pub fn new_title(title: String) -> Result<String, AppError> {
        if title.is_empty() {
            return Err(AppError::BadRequest(
                "model::todo::new_title empty title".to_string(),
            ));
        }

        Ok(title)
    }

    pub fn new_description(description: String) -> Result<String, AppError> {
        Ok(description)
    }

    pub fn new_status(status: i32) -> Result<i32, AppError> {
        if !(0..=3).contains(&status) {
            return Err(AppError::BadRequest(
                "model::todo::new_status range of 0 ~ 3".to_string(),
            ));
        }

        Ok(status)
    }
}
