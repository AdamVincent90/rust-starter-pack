use rust_starter_pack::{
    domain::system::auth::auth::Auth,
    lib::{grpc::server::Tonic, logger::logger::Logger},
};
use sqlx::PgPool;
use tonic::{transport::server::Server, Request, Response, Status};
use user::user_server::{User, UserServer};
use user::{GetUserRequest, GetUserResponse};

pub mod user {
    tonic::include_proto!("user");
}

#[derive(Debug, Default)]
pub struct MyUser {}

#[tonic::async_trait]
impl User for MyUser {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = user::GetUserResponse {
            name: format!("Hello {}!", request.into_inner().user_id).into(),
        };

        Ok(Response::new(reply))
    }
}

pub struct RpcConfig<'a> {
    pub environment: String,
    pub web_address: String,
    pub port: u16,
    pub auth: Auth,
    pub db: PgPool,
    pub log: &'a Logger,
}

pub fn new_rpc(config: RpcConfig) -> Tonic {
    let router = Server::builder().add_service(UserServer::new(MyUser::default()));

    Tonic {
        web_address: config.web_address,
        port: config.port,
        router: router,
    }
}

pub fn load_rpc_services() {
    todo!()
}
