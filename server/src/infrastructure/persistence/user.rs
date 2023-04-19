use async_trait::async_trait;
use sqlx::{Error, MySql, Pool};

use crate::app_error::error::AppError;

use crate::domain::{model::user::User as UserModel, repository::user::User as UserRepository};

#[derive(Debug)]
pub struct User {}

impl User {
    pub fn new() -> User {
        User {}
    }
}

#[async_trait]
impl UserRepository for User {
    async fn list(&self, pool: Pool<MySql>) -> Result<Vec<UserModel>, AppError> {
        let rows = sqlx::query_as::<_, UserModel>(
            r#"
				SELECT
					user_id,
					name,
					birthday,
					email,
					blood_type,
					created_at,
					updated_at,
					is_deleted
				FROM
					user
				WHERE
					is_deleted = FALSE
			"#,
        )
        .fetch_all(&pool)
        .await;

        match rows {
            Ok(users) => Ok(users),
            Err(err) => Err(AppError::Internal(format!(
                "persistence::user::UserRepoImpl::list failed to select user/ {}",
                err,
            ))),
        }
    }

    async fn get(&self, pool: Pool<MySql>, user_id: String) -> Result<UserModel, AppError> {
        let row = sqlx::query_as::<_, UserModel>(
            r#"
				SELECT
					user_id,
					name,
					birthday,
					email,
					blood_type,
					created_at,
					updated_at,
					is_deleted
				FROM
					user
				WHERE
					user_id = ?
					AND is_deleted = FALSE
			"#,
        )
        .bind(&user_id)
        .fetch_one(&pool)
        .await;

        match row {
            Ok(user) => Ok(user),
            Err(Error::RowNotFound) => Err(AppError::NotFound(
                "persistence::user::UserRepoImpl::get user not found/".to_string(),
            )),
            Err(err) => Err(AppError::Internal(format!(
                "persistence::user::UserRepoImpl::list failed to select user/ {}",
                err,
            ))),
        }
    }

    async fn create(&self, pool: Pool<MySql>, user: UserModel) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
				INSERT INTO user (
					user_id, name, birthday, email, blood_type, created_at, updated_at
				) VALUES (
					?, ?, ?, ?, ?, ?, ?
				)
			"#,
        )
        .bind(&user.user_id)
        .bind(&user.name)
        .bind(user.birthday)
        .bind(&user.email)
        .bind(user.blood_type)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(AppError::Internal(format!(
                "persistence::user::UserRepoImpl::create failed to create/ {}",
                err,
            ))),
        }
    }

    async fn update(&self, pool: Pool<MySql>, user: UserModel) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
				UPDATE
					user
				SET
					name = ?,
					updated_at = ?
				WHERE
					user_id = ?
					AND is_deleted = FALSE
			"#,
        )
        .bind(&user.name)
        .bind(user.updated_at)
        .bind(&user.user_id)
        .execute(&pool)
        .await;

        match result {
            Ok(result) => {
                if result.rows_affected() != 0 {
                    Ok(())
                } else {
                    Err(AppError::NotFound(
                        "persistence::user::UserRepoImpl::update user not found".to_string(),
                    ))
                }
            }
            Err(err) => Err(AppError::Internal(format!(
                "persistence::user::UserRepoImpl::update failed to update user/ {}",
                err,
            ))),
        }
    }

    async fn delete(&self, pool: Pool<MySql>, user_id: String, now: i32) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
				UPDATE
					user
				SET
					updated_at = ?,
					is_deleted = TRUE
				WHERE
					user_id = ?
					AND is_deleted = FALSE
			"#,
        )
        .bind(now)
        .bind(&user_id)
        .execute(&pool)
        .await;

        match result {
            Ok(result) => {
                if result.rows_affected() != 0 {
                    Ok(())
                } else {
                    Err(AppError::NotFound(
                        "persistence::user::UserRepoImpl::delete user not found".to_string(),
                    ))
                }
            }
            Err(err) => Err(AppError::Internal(format!(
                "persistence::user::UserRepoImpl::delete failed to delete user/ {}",
                err,
            ))),
        }
    }
}
