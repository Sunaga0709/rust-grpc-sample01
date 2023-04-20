use tonic::{Request, Response, Status};

use crate::gen::grpc_api::greet_v1::greet_service_server;
use crate::gen::grpc_api::greet_v1::{GreetRequest, GreetResponse};

#[derive(Default)]
pub struct GreetService {}

impl GreetService {
    pub fn new() -> GreetService {
        GreetService {}
    }
}

#[tonic::async_trait]
impl greet_service_server::GreetService for GreetService {
    async fn greet(&self, req: Request<GreetRequest>) -> Result<Response<GreetResponse>, Status> {
        let payload = req.into_inner();
        let text = format!("Hello, {}!", payload.name);

        Ok(Response::new(GreetResponse { text }))
    }
}
