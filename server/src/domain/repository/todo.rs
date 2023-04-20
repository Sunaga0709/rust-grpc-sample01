use async_trait::async_trait;
use sqlx::{MySql, Pool};

use crate::app_error::error::AppError;
use crate::domain::model::todo::Todo as TodoModel;

#[async_trait]
pub trait Todo {
    async fn list(&self, pool: Pool<MySql>, user_id: String) -> Result<Vec<TodoModel>, AppError>;
    async fn get(
        &self,
        pool: Pool<MySql>,
        user_id: String,
        todo_id: String,
    ) -> Result<TodoModel, AppError>;
    async fn create(&self, pool: Pool<MySql>, todo: TodoModel) -> Result<(), AppError>;
    async fn update(&self, pool: Pool<MySql>, todo: TodoModel) -> Result<(), AppError>;
}

impl std::fmt::Debug for dyn Todo + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "custom debug implementation TodoRepository")
    }
}
