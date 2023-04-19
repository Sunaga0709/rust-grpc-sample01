use async_trait::async_trait;
use sqlx::{MySql, Pool};

use crate::app_error::error::AppError;

use crate::domain::model::user::User as UserModel;

#[async_trait]
pub trait User {
    async fn list(&self, pool: Pool<MySql>) -> Result<Vec<UserModel>, AppError>;
    async fn get(&self, pool: Pool<MySql>, user_id: String) -> Result<UserModel, AppError>;
    async fn create(&self, pool: Pool<MySql>, user: UserModel) -> Result<(), AppError>;
    async fn update(&self, pool: Pool<MySql>, user: UserModel) -> Result<(), AppError>;
    async fn delete(&self, pool: Pool<MySql>, user_id: String, now: i32) -> Result<(), AppError>;
}

impl std::fmt::Debug for dyn User + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "My custom debug implementation")
    }
}
