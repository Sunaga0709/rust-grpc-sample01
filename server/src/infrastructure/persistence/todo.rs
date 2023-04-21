use async_trait::async_trait;
use sqlx::{mysql::MySql, Error, Pool};

use crate::app_error::error::AppError;
use crate::domain::{
    model::{comment::Comment as CommentModel, todo::Todo as TodoModel},
    repository::{
        db_conn::DBConn,
        todo::Todo as TodoRepository
    },
};


#[derive(Debug)]
pub struct Todo {}

impl Todo {
    pub fn new() -> Self {
        Todo {}
    }
}

#[async_trait]
impl TodoRepository for Todo {
    async fn list(&self, conn: dyn DBConn, user_id: String) -> Result<Vec<TodoModel>, AppError> {
        let result = conn.query_with_params(
            r#"
                SELECT
                    todo_id,
                    user_id,
                    title,
                    description,
                    status,
                    created_at,
                    updated_at
                FROM
                    todo
                WHERE
                    user_id = ?
                    AND is_deleted = FALSE
            "#,
            &[&user_id]
        ).await;

        match result {
            Ok(rows) => {
                let todos = rows
                    .iter()
                    .map(|todo| TodoModel {
                        todo_id: todo.get("todo_id"),
                        user_id: todo.get("user_id"),
                        title: todo.get("title"),
                        description: todo.get("description"),
                        status: todo.get("status"),
                        created_at: todo.get("created_at"),
                        updated_at: todo.get("updated_at"),
                    })
                    .collect();
                Ok(todos)
            },
            Err(err) => {
                Err(AppError::Internal(
                    format!("persistence::todo::Todo::list failed to select todos/ {}", err),
                ))
            },
        }
    }

    async fn get(
        &self,
        conn: dyn DBConn,
        user_id: String,
        todo_id: String,
    ) -> Result<TodoModel, AppError> {
        let result = conn.query_one_with_params(
            r#"
                SELECT
                    todo_id,
                    user_id,
                    title,
                    description,
                    status,
                    created_at,
                    updated_at
                FROM
                    todo
                WHERE
                    user_id = ?
                    AND todo_id = ?
                    AND is_deleted = FALSE
            "#,
            &[&user_id, &todo_id],
        ).await;
        
        match result {
            Ok(row) => {
                Ok(TodoModel {
                    todo_id: row.get("todo_id"),
                    user_id: row.get("user_id"),
                    title: row.get("titile"),
                    description: row.get("description"),
                    status: row.get("status"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                })
            },
            Err(Error::RowNotFound) => {
                Err(AppError::NotFound(
                    "persistence::todo::Todo::get todo not found".to_string()
                ))
            },
            Err(err) => {
                Err(AppError::Internal(
                    format!(
                        "persistence::todo::Todo::get failed to get todo/ {}", err
                    )
                ))
            }
        }
    }

    async fn create(&self, conn: dyn DBConn, todo: TodoModel) -> Result<(), AppError> {
        let result = conn.execute_with_params(
            r#"
				INSERT INTO todo (
					todo_id, user_id, title, description, status, created_at, updated_at
				) VALUES (
					?, ?, ?, ?, ?, ?, ?
				)
			"#,
            &[&todo.todo_id, &todo.user_id, &todo.title, &todo.description, &todo.status, &todo.created_at, &todo.updated_at]
        ).await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                Err(AppError::Internal(format!("persistence::todo::Todo::create failed to create todo/ {}", err)))
            }
        }
    }

    async fn update(&self, conn: dyn DBConn, todo: TodoModel) -> Result<(), AppError> {
        let result = conn.execute_with_params(
            r#"
                UPDATE
                    todo
                SET
                    title = ?,
                    description = ?,
                    status = ?,
                    updated_at = ?
                WHERE
                    todo_id = ?
                    AND user_id = ?
                    AND is_deleted = FALSE
            "#,
            &[&todo.title, &todo.description, &todo.status, &todo.updated_at, &todo.todo_id, &todo.user_id]
        ).await;

        match result {
            Ok(rows) => {
                if rows == 1 {
                    return Ok(())
                }
                Err(AppError::NotFound("persistence::todo::Todo::update todo not found".to_string()))
            },
            Err(err) => {
                Err(AppError::Internal(
                    format!("persistence::todo::Todo::update failed to update todo/ {}", err)
                ))
            }
        }
    }

    async fn delete(
        &self,
        conn: dyn DBConn,
        user_id: String,
        todo_id: String,
        now: i32,
    ) -> Result<(), AppError> {
        let result = conn.execute_with_params(
            r#"
                UPDATE
                    todo
                SET
                    updated_at = ?,
                    is_deleted = TRUE
                WHERE
                    user_id = ?
                    AND todo_id = ?
                    AND is_deleted = FALSE
            "#,
            &[&now, &user_id, &todo_id],
        ).await;

        match result {
            Ok(rows) => {
                if rows == 1 {
                    return Ok(())
                }
                Err(AppError::NotFound(
                    "persistence::todo::Todo::delete todo not found".to_string()
                ))
            },
            Err(err) => {
                Err(AppError::Internal(
                    format!("persistence::todo::Todo::delete failed to delete todo/ {}", err)
                ))
            }
        }
    }

    async fn create_comment(
        &self,
        conn: dyn DBConn,
        comment: CommentModel,
    ) -> Result<(), AppError> {
        let result = conn.execute_with_params(
            r#"
                INSERT INTO comment (
                    comment_id, todo_id, text, created_at
                ) VALUES (
                    ?, ?, ?, ?
                )
            "#,
            &[&comment.comment_id, &comment.todo_id, &comment.text, &comment.created_at],
        ).await;

        match result {
            Ok(rows) => Ok(()),
            Err(err) => {
                Err(AppError::Internal(
                    format!("persistence::todo::Todo::create_comment failed to create comment/ {}", err)
                ))
            }
        }
    }
}
