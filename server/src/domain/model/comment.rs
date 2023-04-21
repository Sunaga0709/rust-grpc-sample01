use sqlx::{Encode, FromRow};
use uuid::Uuid;

use crate::app_error::error::AppError;
use crate::util::datetime;

#[derive(Clone, Debug, Encode, FromRow)]
pub struct Comment {
    pub comment_id: String,
    pub todo_id: String,
    pub text: String,
    pub created_at: i32,
    pub is_deleted: Option<bool>,
}
impl Comment {
    pub fn new(todo_id: String, text: String) -> Self {
        Comment {
            comment_id: Uuid::new_v4().to_string(),
            todo_id,
            text,
            created_at: datetime::get_timestamp(),
            is_deleted: Some(false),
        }
    }

    pub fn new_todo_id(id: String) -> Result<String, AppError> {
        if id.is_empty() {
            return Err(AppError::BadRequest(
                "model::comment::Comment::new_todo_id empty todo_id".to_string(),
            ));
        }

        Ok(id)
    }

    pub fn new_text(text: String) -> Result<String, AppError> {
        if text.is_empty() {
            return Err(AppError::BadRequest(
                "model::comment::Comment::new_text empty text".to_string(),
            ));
        }

        Ok(text)
    }
}
