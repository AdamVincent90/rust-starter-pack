use rust_starter_pack::{
    domain::{
        system::auth::auth::{Auth, StandardClaims},
        web::state::state::{MuxState, SharedState},
    },
    lib::{grpc::server::Tonic, logger::logger::Logger},
};
use sqlx::PgPool;
use tokio::sync::RwLock;
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
    // Initialise our global web state that is shared across the project.
    // Global state uses A mutex to safely read and write to the state without any side effects.
    let _global_state = SharedState::new(RwLock::new(MuxState {
        environment: config.environment.clone(),
        claims: StandardClaims::default(),
    }));

    // let router = Server::builder()
    //     .layer(
    //         // We use ServiceBuilder as this means that the order of middleware is from top to bottom.
    //         ServiceBuilder::new()
    //             // * Logging
    //             .layer(middleware::from_fn_with_state(
    //                 LoggingContext {
    //                     log: config.log.clone(),
    //                 },
    //                 logging,
    //             ))
    //             // * Error handling
    //             .layer(middleware::from_fn_with_state(
    //                 ErrorContext {
    //                     log: config.log.clone(),
    //                 },
    //                 error,
    //             ))
    //             // * Authentication
    //             .layer(middleware::from_fn_with_state(
    //                 AuthContext { auth: config.auth },
    //                 authenticate,
    //             ))
    //             // * Auditing
    //             .layer(middleware::from_fn_with_state(
    //                 AuditContext {
    //                     db: config.db.clone(),
    //                 },
    //                 audit,
    //             )),
    //     )
    //     .layer(Extension(global_state))
    //     .add_service(UserServer::new(MyUser::default()));

    let r1 = Server::builder().add_service(UserServer::new(MyUser::default()));

    Tonic {
        web_address: config.web_address,
        port: config.port,
        router: r1,
    }
}
