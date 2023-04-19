use tonic::{Request, Response, Status};
use sqlx::{mysql::MySql, Pool};

use crate::domain::repository::user::User as UserRepository;
use crate::gen::grpc_api::user_v1::{
    user_service_server, CreateUserRequest, CreateUserResponse, DeleteUserRequest,
    DeleteUserResponse, GetUserRequest, GetUserResponse, ListUserRequest, ListUserResponse,
    UpdateUserRequest, UpdateUserResponse, User as UserMessage,
};
use crate::usecase::user::User as UserUsecase;

use crate::util::datetime;

pub struct UserService {
    repo:  Box<dyn UserRepository + Send + Sync + 'static>,
    uc: UserUsecase,
    pool: Pool<MySql>,
}

impl UserService {
    pub fn new(
        repo:  Box<dyn UserRepository + Send + Sync + 'static>,
        uc: UserUsecase,
        pool: Pool<MySql>,
    ) -> UserService {
        UserService { repo, uc, pool }
    }
}

#[tonic::async_trait]
impl user_service_server::UserService for UserService {
    async fn list_user(
        &self,
        _: Request<ListUserRequest>,
    ) -> Result<Response<ListUserResponse>, Status> {
        match self.repo.list(self.pool.clone()).await {
            Ok(result_users) => {
                let users: Vec<UserMessage> = result_users
                    .iter()
                    .map(|user| UserMessage {
                        user_id: user.user_id.clone(),
                        name: user.name.clone(),
                        birthday: user.birthday,
                        email: user.email.clone(),
                        blood_type: user.blood_type,
                        created_at: user.created_at,
                        updated_at: user.updated_at,
                    })
                    .collect();

                Ok(Response::new(ListUserResponse { users }))
            }
            Err(err) => Err(Status::from(err)),
        }
    }

    async fn get_user(
        &self,
        req: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let user_id = req.into_inner().user_id;

        match self.repo.get(self.pool.clone(), user_id).await {
            Ok(result) => {
                let user = UserMessage {
                    user_id: result.user_id.clone(),
                    name: result.name.clone(),
                    birthday: result.birthday,
                    email: result.email.clone(),
                    blood_type: result.blood_type,
                    created_at: result.created_at,
                    updated_at: result.updated_at,
                };
                Ok(Response::new(GetUserResponse { user: Some(user) }))
            }
            Err(err) => Err(Status::from(err)),
        }
    }

    async fn create_user(
        &self,
        req: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let param = req.into_inner();
        let name = param.clone().name;
        let birthday = param.birthday;
        let email = param.clone().email;
        let blood_type = param.blood_type;

        match self.uc.create(name, birthday, email, blood_type).await {
            Ok(user_id) => Ok(Response::new(CreateUserResponse { user_id })),
            Err(err) => Err(Status::from(err)),
        }
    }

    async fn update_user(
        &self,
        req: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserResponse>, Status> {
        let param = req.into_inner();
        let user_id = param.clone().user_id;
        let name = param.clone().name;

        match self.uc.update(user_id, name).await {
            Ok(_) => {
                let message = "success".to_string();
                Ok(Response::new(UpdateUserResponse { message }))
            }
            Err(err) => Err(Status::from(err)),
        }
    }

    async fn delete_user(
        &self,
        req: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        let user_id = req.into_inner().user_id;
        let now = datetime::get_timestamp();

        match self.repo.delete(self.pool.clone(), user_id, now).await {
            Ok(_) => {
                let message = "success".to_string();
                Ok(Response::new(DeleteUserResponse { message }))
            }
            Err(err) => Err(Status::from(err)),
        }
    }
}
