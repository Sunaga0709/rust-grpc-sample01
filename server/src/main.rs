use sqlx::mysql::MySqlPoolOptions;
use tonic::transport::Server;
use tower::ServiceBuilder;

mod app_error;
mod domain;
mod gen;
mod infrastructure;
mod interceptor;
mod service;
mod usecase;
mod util;

use crate::interceptor::logger::LoggingMiddlewareLayer;
// use crate::interceptor::logger::{logging_interceptor, logging_response_interceptor};

use gen::grpc_api::{
    greet_v1::greet_service_server, todo_v1::todo_service_server, user_v1::user_service_server,
};

use service::greet::GreetService;

use infrastructure::persistence::{todo::Todo as TodoRepoImpl, user::User as UserRepoImpl};
use service::{todo::TodoService, user::UserService};
use usecase::{todo::Todo as TodoUsecase, user::User as UserUsecase};

use dotenv::dotenv;
use log::info;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init();

    // DBに接続
    let db_url = env::var("DATABASE_URL").unwrap();
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let server_address = env::var("BASE_URL")
        .expect("Failed to load BASE_URL, must be set")
        .parse()?;

    // GreetService
    let greet_service = greet_service_server::GreetServiceServer::new(GreetService::new());
    let greet_with_logger = ServiceBuilder::new()
        .layer(LoggingMiddlewareLayer)
        .service(greet_service);

    // TodoService
    let todo_service = todo_service_server::TodoServiceServer::new(TodoService::new(
        Box::new(TodoRepoImpl::new()),
        TodoUsecase::new(Box::new(TodoRepoImpl::new()), pool.clone()),
        pool.clone(),
    ));
    let todo_with_logger = ServiceBuilder::new()
        .layer(LoggingMiddlewareLayer)
        .service(todo_service);

    // UserService
    let user_service = user_service_server::UserServiceServer::new(UserService::new(
        Box::new(UserRepoImpl::new()),
        UserUsecase::new(Box::new(UserRepoImpl::new()), pool.clone()),
        pool.clone(),
    ));
    let user_with_logger = ServiceBuilder::new()
        .layer(LoggingMiddlewareLayer)
        .service(user_service);

    info!("start server with {}", server_address);
    Server::builder()
        .concurrency_limit_per_connection(256)
        .add_service(greet_with_logger)
        .add_service(todo_with_logger)
        .add_service(user_with_logger)
        .serve(server_address)
        .await?;

    Ok(())
}

fn init() {
    dotenv().expect("Failed to load .env");
    env_logger::init();
}
