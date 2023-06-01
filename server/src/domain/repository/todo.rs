use async_trait::async_trait;

use crate::app_error::error::AppError;
use crate::domain::model::{comment::Comment as CommentModel, todo::Todo as TodoModel};

use super::db_conn::DBConn;

#[async_trait]
pub trait Todo {
    // todo
    async fn list(&self, conn: Box<dyn DBConn + Send + Sync + 'static>, user_id: String) -> Result<Vec<TodoModel>, AppError>;
    async fn get(
        &self,
        conn: Box<dyn DBConn + Send + Sync + 'static>,
        user_id: String,
        todo_id: String,
    ) -> Result<TodoModel, AppError>;
    async fn create(&self, conn: Box<dyn DBConn + Send + Sync + 'static>, todo: TodoModel) -> Result<(), AppError>;
    async fn update(&self, conn: Box<dyn DBConn + Send + Sync + 'static>, todo: TodoModel) -> Result<(), AppError>;
    async fn delete(
        &self,
        conn: Box<dyn DBConn + Send + Sync + 'static>,
        user_id: String,
        todo_id: String,
        now: i32,
    ) -> Result<(), AppError>;

    // comment
    async fn create_comment(
        &self,
        conn: Box<dyn DBConn + Send + Sync + 'static>,
        comment: CommentModel,
    ) -> Result<(), AppError>;
}

impl std::fmt::Debug for dyn Todo + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "custom debug implementation TodoRepository")
    }
}
