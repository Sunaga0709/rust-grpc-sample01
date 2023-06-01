use async_trait::async_trait;
use sqlx::{Error, Row};

use crate::app_error::error::AppError;
use crate::domain::{model::user::User as UserModel, repository::{db_conn::DBConn, user::User as UserRepository}};

#[derive(Debug)]
pub struct User {}

impl User {
    pub fn new() -> Self {
        User {}
    }
}

#[async_trait]
impl UserRepository for User {
    async fn list(&self, conn: Box<dyn DBConn>) -> Result<Vec<UserModel>, AppError> {
        let result = conn.query(
            r#"
				SELECT
					user_id,
					name,
					birthday,
					email,
					blood_type,
					created_at,
					updated_at
				FROM
					user
				WHERE
					is_deleted = FALSE
			"#,
        )
        .await;

        match result {
            Ok(rows) => { 
                let users = rows.iter()
                    .map(|r| UserModel {
                        user_id: r.get("user_id"),
                        name: r.get("name"),
                        birthday: r.get("birthday"),
                        email: r.get("email"),
                        blood_type: r.get("blood_type"),
                        created_at: r.get("created_at"),
                        updated_at: r.get("updated_at"),
                    })
                    .collect();
                Ok(users)
            },
            Err(err) => {
                Err(AppError::Internal(format!("persistence::user::User::list failed to select users/ {}", err)))
            }
        }
    }

    async fn get(&self, conn: Box<dyn DBConn>, user_id: String) -> Result<UserModel, AppError> {
        let result = conn.query_one_with_params(
            r#"
				SELECT
					user_id,
					name,
					birthday,
					email,
					blood_type,
					created_at,
					updated_at
				FROM
					user
				WHERE
					user_id = ?
					AND is_deleted = FALSE
			"#,
            &[&user_id],
        ).await;

        match result {
            Ok(row) => {
                Ok(UserModel {
                    user_id: row.get("user_id"),
                    name: row.get("name"),
                    birthday: row.get("birthday"),
                    email: row.get("email"),
                    blood_type: row.get("blood_type"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                })
            },
            Err(Error::RowNotFound) => {
                Err(AppError::NotFound("persistence::user::User::get user not found".to_string()))
            },
            Err(err) => {
                Err(AppError::Internal(
                    format!("persistence::user::User::get failed to select user/ {}", err)
                ))
            }
        }
    }

    async fn create(&self, conn: Box<dyn DBConn>, user: UserModel) -> Result<(), AppError> {
        let result = conn.execute_with_params(
            r#"
				INSERT INTO user (
					user_id, name, birthday, email, blood_type, created_at, updated_at
				) VALUES (
					?, ?, ?, ?, ?, ?, ?
				)
			"#,
            &[&user.user_id, &user.name, &user.email, &user.blood_type, &user.created_at, &user.updated_at],
        ).await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                Err(AppError::Internal(
                    format!("persistence::user::User::create failed to create user/ {}", err)
                ))
            }
        }
    }

    async fn update(&self, conn: Box<dyn DBConn>, user: UserModel) -> Result<(), AppError> {
        let result = conn.execute_with_params(
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
            &[&user.name, &user.updated_at, &user.user_id],
        ).await;

        match result {
            Ok(rows) => {
                if rows == 1 {
                    return Ok(())
                }
                Err(AppError::NotFound("persistece::user::User::update user not found".to_string()))
            },
            Err(err) => {
                Err(AppError::Internal(
                    format!("persistence::user::User::update failed to update user/ {}", err)
                ))
            }
        }
    }

    async fn delete(&self, conn: Box<dyn DBConn>, user_id: String, now: i32) -> Result<(), AppError> {
        let result = conn.execute_with_params(
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
            &[&now, &user_id],
        ).await;

        match result {
            Ok(rows) => {
                if rows == 1 {
                    return Ok(())
                }
                Err(AppError::NotFound("persistence::user::User::delete user not found".to_string()))
            },
            Err(err) => {
                Err(AppError::Internal(
                    format!("persistence::user::User::delete failed to delete user/ {}", err)
                ))
            }
        }
    }
}
