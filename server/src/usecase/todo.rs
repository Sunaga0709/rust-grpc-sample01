use crate::app_error::error::AppError;
use crate::domain::{
    model::{comment::Comment as CommentModel, todo::Todo as TodoModel},
    repository::{
        db_conn::DBConn,
        todo::Todo as TodoRepository,
    },
};

#[derive(Debug)]
pub struct Todo {
    repo: Box<dyn TodoRepository + Send + Sync>,
    conn: Box<dyn DBConn + Send + Sync + 'static>,
}

impl Todo {
    // todo
    pub fn new(repo: Box<dyn TodoRepository + Send + Sync>, conn: Box<dyn DBConn + Send + Sync + 'static>) -> Self {
        Todo { repo, conn }
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

        match self.repo.create(self.conn, _new_todo.clone()).await {
            Ok(_) => Ok(_new_todo.clone().todo_id),
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

        match self.repo.update(self.conn, _new_todo.clone()).await {
            Ok(()) => Ok(()),
            Err(err) => Err(err),
        }
    }

    // comment
    pub async fn create_comment(&self, todo_id: String, text: String) -> Result<String, AppError> {
        let todo_id = CommentModel::new_todo_id(todo_id)?;
        let text = CommentModel::new_text(text)?;
        let _new_comment = CommentModel::new(todo_id, text);

        match self
            .repo
            .create_comment(self.conn, _new_comment.clone())
            .await
        {
            Ok(_) => Ok(_new_comment.clone().comment_id),
            Err(err) => Err(err),
        }
    }
}
