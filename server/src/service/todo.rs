use tonic::{Request, Response, Status};

use crate::domain::repository::{
    db_conn::DBConn,
    todo::Todo as TodoRepository
};
use crate::gen::grpc_api::todo_v1::{
    todo_service_server, CreateCommentRequest, CreateCommentResponse, CreateTodoRequest,
    CreateTodoResponse, DeleteTodoRequest, DeleteTodoResponse, GetTodoRequest, GetTodoResponse,
    ListTodoRequest, ListTodoResponse, Todo as TodoMessage, UpdateTodoRequest, UpdateTodoResponse,
};
use crate::usecase::todo::Todo as TodoUsecase;
use crate::util::datetime;

pub struct TodoService {
    repo: Box<dyn TodoRepository + Send + Sync + 'static>,
    uc: TodoUsecase,
    conn: Box<dyn DBConn + Send + Sync + 'static>,
}

impl TodoService {
    pub fn new(
        repo: Box<dyn TodoRepository + Send + Sync + 'static>,
        uc: TodoUsecase,
        conn: Box<dyn DBConn + Send + Sync + 'static>,
    ) -> Self {
        TodoService { repo, uc, conn }
    }
}

#[tonic::async_trait]
impl todo_service_server::TodoService for TodoService {
    async fn list_todo(
        &self,
        _: Request<ListTodoRequest>,
    ) -> Result<Response<ListTodoResponse>, Status> {
        // TODO: 後で認証を入れたい
        let user_id = "testUserId1".to_string();

        match self.repo.list(self.conn, user_id).await {
            Ok(result_todo) => {
                let todos: Vec<TodoMessage> = result_todo
                    .iter()
                    .map(|todo| TodoMessage {
                        todo_id: todo.todo_id.clone(),
                        title: todo.title.clone(),
                        description: Some(todo.description.clone()),
                        status: todo.status,
                        created_at: todo.created_at,
                        updated_at: todo.updated_at,
                    })
                    .collect();

                Ok(Response::new(ListTodoResponse { todos }))
            }
            Err(err) => Err(Status::from(err)),
        }
    }

    async fn get_todo(
        &self,
        req: Request<GetTodoRequest>,
    ) -> Result<Response<GetTodoResponse>, Status> {
        // TODO: 後で認証入れたい
        let user_id = "testUserId1".to_string();

        match self
            .repo
            .get(self.conn, user_id, req.into_inner().todo_id)
            .await
        {
            Ok(result) => {
                let todo = Some(TodoMessage {
                    todo_id: result.todo_id.clone(),
                    title: result.title.clone(),
                    description: Some(result.description.clone()),
                    status: result.status,
                    created_at: result.created_at,
                    updated_at: result.updated_at,
                });

                Ok(Response::new(GetTodoResponse { todo }))
            }
            Err(err) => Err(Status::from(err)),
        }
    }

    async fn create_todo(
        &self,
        req: Request<CreateTodoRequest>,
    ) -> Result<Response<CreateTodoResponse>, Status> {
        // TODO: 後で認証入れたい
        let user_id = "testUserId1".to_string();
        let _param = req.into_inner();

        match self
            .uc
            .create(user_id, _param.clone().title, _param.clone().description)
            .await
        {
            Ok(todo_id) => Ok(Response::new(CreateTodoResponse { todo_id })),
            Err(err) => Err(Status::from(err)),
        }
    }

    async fn update_todo(
        &self,
        req: Request<UpdateTodoRequest>,
    ) -> Result<Response<UpdateTodoResponse>, Status> {
        // TODO: 後で認証入れたい
        let user_id = "testUserId1".to_string();
        let param = req.into_inner();

        match self
            .uc
            .update(
                param.clone().todo_id,
                user_id,
                param.clone().title,
                param.clone().description,
                param.clone().status,
            )
            .await
        {
            Ok(_) => Ok(Response::new(UpdateTodoResponse {
                message: "success".to_string(),
            })),
            Err(err) => Err(Status::from(err)),
        }
    }

    async fn delete_todo(
        &self,
        req: Request<DeleteTodoRequest>,
    ) -> Result<Response<DeleteTodoResponse>, Status> {
        // TODO: 後で認証入れたい
        let user_id = "testUserId1".to_string();

        match self
            .repo
            .delete(
                self.conn,
                user_id,
                req.into_inner().todo_id,
                datetime::get_timestamp(),
            )
            .await
        {
            Ok(()) => {
                let message = "success".to_string();
                Ok(Response::new(DeleteTodoResponse { message }))
            }
            Err(err) => Err(Status::from(err)),
        }
    }

    async fn create_comment(
        &self,
        req: Request<CreateCommentRequest>,
    ) -> Result<Response<CreateCommentResponse>, Status> {
        let param = req.into_inner();

        match self
            .uc
            .create_comment(param.clone().todo_id, param.clone().text)
            .await
        {
            Ok(comment_id) => Ok(Response::new(CreateCommentResponse { comment_id })),
            Err(err) => Err(Status::from(err)),
        }
    }
}
