use rust_starter_pack::{
    core::user::user,
    domain::system::auth::auth::Auth,
    lib::{grpc::server::Tonic, logger::logger::Logger},
};
use sqlx::PgPool;
use tonic::transport::{server::Router, Server};

use crate::rpc::servers::user::UserContext;

use super::servers::user::user::user_server::UserServer;

pub struct RpcConfig<'a> {
    pub environment: String,
    pub web_address: String,
    pub port: u16,
    pub auth: Auth,
    pub db: PgPool,
    pub log: &'a Logger,
}

pub fn new_rpc(config: RpcConfig) -> Tonic {
    let router = load_rpc_services(&config);

    Tonic {
        web_address: config.web_address,
        port: config.port,
        router: router,
    }
}

pub fn load_rpc_services(config: &RpcConfig) -> Router {
    // Create user handler that will acts as the context for users routes.
    let user_context = UserContext {
        version: String::from("v1"),
        user_core: user::new_core(&config.log, &config.db),
    };

    let router = Server::builder().add_service(UserServer::new(user_context));

    router
}
