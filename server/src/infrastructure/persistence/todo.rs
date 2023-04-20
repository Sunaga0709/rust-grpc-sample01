use async_trait::async_trait;
use sqlx::{mysql::MySql, Error, Pool};

use crate::app_error::error::AppError;
use crate::domain::{model::todo::Todo as TodoModel, repository::todo::Todo as TodoRepository};

#[derive(Debug)]
pub struct Todo {}

impl Todo {
    pub fn new() -> Self {
        Todo {}
    }
}

#[async_trait]
impl TodoRepository for Todo {
    async fn list(&self, pool: Pool<MySql>, user_id: String) -> Result<Vec<TodoModel>, AppError> {
        let result = sqlx::query_as::<_, TodoModel>(
            r#"
				SELECT
					todo_id,
					user_id,
					title,
					description,
					status,
					created_at,
					updated_at,
				FROM
					todo
				WHERE
					user_id = ?
					AND is_deleted = FALSE
			"#,
        )
        .bind(&user_id)
        .fetch_all(&pool)
        .await;

        match result {
            Ok(todos) => Ok(todos),
            Err(err) => Err(AppError::Internal(
                format!("persistence::todo::Todo::list failed to get todos/ {}", err,).to_string(),
            )),
        }
    }

    async fn get(
        &self,
        pool: Pool<MySql>,
        user_id: String,
        todo_id: String,
    ) -> Result<TodoModel, AppError> {
        let result = sqlx::query_as::<_, TodoModel>(
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
        )
        .bind(&user_id)
        .bind(&todo_id)
        .fetch_one(&pool)
        .await;

        match result {
            Ok(todo) => Ok(todo),
            Err(Error::RowNotFound) => Err(AppError::NotFound(
                "persistence::todo::Todo::get not found todo".to_string(),
            )),
            Err(err) => Err(AppError::Internal(
                format!("persistence::todo::Todo::get failed to get todo/ {}", err,).to_string(),
            )),
        }
    }

    async fn create(&self, pool: Pool<MySql>, todo: TodoModel) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
				INSERT INTO todo (
					todo_id, user_id, title, description, status, created_at, updated_at
				) VALUES (
					?, ?, ?, ?, ?, ?, ?
				)
			"#,
        )
        .bind(&todo.todo_id)
        .bind(&todo.user_id)
        .bind(&todo.title)
        .bind(&todo.description)
        .bind(&todo.status)
        .bind(&todo.created_at)
        .bind(&todo.updated_at)
        .execute(&pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(AppError::Internal(
                format!(
                    "persistence::todo::Todo::create failed to create todo/ {}",
                    err,
                )
                .to_string(),
            )),
        }
    }

    async fn update(&self, pool: Pool<MySql>, todo: TodoModel) -> Result<(), AppError> {
        let result = sqlx::query(
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
        )
        .bind(&todo.title)
        .bind(&todo.description)
        .bind(&todo.status)
        .bind(&todo.updated_at)
        .bind(&todo.todo_id)
        .bind(&todo.user_id)
        .execute(&pool)
        .await;

        match result {
            Ok(result) => {
                if result.rows_affected() == 1 {
                    Ok(())
                } else {
                    Err(AppError::NotFound(
                        "persistence::todo::Todo::update todo not found".to_string(),
                    ))
                }
            }
            Err(err) => Err(AppError::Internal(
                format!(
                    "persistence::todo::Todo::update failed to update todo/ {}",
                    err
                )
                .to_string(),
            )),
        }
    }
}
