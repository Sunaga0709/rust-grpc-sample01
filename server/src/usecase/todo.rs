use sqlx::{mysql::MySql, Pool};

use crate::app_error::error::AppError;
use crate::domain::{model::todo::Todo as TodoModel, repository::todo::Todo as TodoRepository};

#[derive(Debug)]
pub struct Todo {
    repo: Box<dyn TodoRepository + Send + Sync>,
    pool: Pool<MySql>,
}

impl Todo {
    pub fn new(repo: Box<dyn TodoRepository + Send + Sync>, pool: Pool<MySql>) -> Self {
        Todo { repo, pool }
    }

    pub async fn create(
        &self,
        user_id: String,
        title: String,
        description: Option<String>,
    ) -> Result<String, AppError> {
        let title = TodoModel::new_title(title)?;
        let description = match description {
            Some(desc) => TodoModel::new_description(desc)?,
            None => String::new(),
        };
        let _new_todo = TodoModel::new_create(user_id, title, description);

        match self.repo.create(self.pool.clone(), _new_todo.clone()).await {
            Ok(_) => Ok(String::from(_new_todo.todo_id.clone())),
            Err(err) => Err(err),
        }
    }

    pub async fn update(
        &self,
        todo_id: String,
        user_id: String,
        title: String,
        description: Option<String>,
        status: i32,
    ) -> Result<(), AppError> {
        let title = TodoModel::new_title(title)?;
        let description = match description {
            Some(desc) => TodoModel::new_description(desc)?,
            None => "".to_string(),
        };
        let status = TodoModel::new_status(status)?;
        let _new_todo = TodoModel::new_update(todo_id, user_id, title, description, status);

        match self.repo.update(self.pool.clone(), _new_todo.clone()).await {
            Ok(()) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
