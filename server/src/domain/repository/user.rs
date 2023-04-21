use async_trait::async_trait;

use crate::app_error::error::AppError;
use crate::domain::model::user::User as UserModel;

use super::db_conn::DBConn;

#[async_trait]
pub trait User {
    async fn list(&self, conn: Box<dyn DBConn>) -> Result<Vec<UserModel>, AppError>;
    async fn get(&self, conn: Box<dyn DBConn>, user_id: String) -> Result<UserModel, AppError>;
    async fn create(&self, conn: Box<dyn DBConn>, user: UserModel) -> Result<(), AppError>;
    async fn update(&self, conn: Box<dyn DBConn>, user: UserModel) -> Result<(), AppError>;
    async fn delete(&self, conn: Box<dyn DBConn>, user_id: String, now: i32) -> Result<(), AppError>;
}

impl std::fmt::Debug for dyn User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "custom debug implementation UserRepository")
    }
}
