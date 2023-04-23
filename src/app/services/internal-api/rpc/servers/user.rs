use rust_starter_pack::core::user::user::UserCore;
use rust_starter_pack::domain::web::state::state::MuxState;
use tonic::{Request, Response, Status};
use user::user_server::User;
use user::{GetUserRequest, GetUserResponse};

pub mod user {
    tonic::include_proto!("user");
}

// UserContext contains any state required when it comes to working with user operations.
#[derive(Clone)]
pub struct UserContext {
    pub version: String,
    pub user_core: UserCore,
}

#[tonic::async_trait]
impl User for UserContext {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        println!("Got a request: {:?}", request);

        let _response = match self.user_core.get_all(&MuxState::default()).await {
            Ok(response) => response,
            Err(err) => return Err(Status::aborted(err.message)),
        };

        let reply = user::GetUserResponse {
            name: format!("Hello {}!", request.into_inner().user_id).into(),
        };

        Ok(Response::new(reply))
    }
}
