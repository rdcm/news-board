use crate::app_state::AppState;
use crate::auth_generated::auth_service_server::AuthService;
use crate::auth_generated::{
    SignInRequest, SignInResponse, SignOutRequest, SignOutResponse, SignUpRequest, SignUpResponse,
};
use crate::infrastructure::{create_user, save_session_id};
use crate::utils::{generate_password_hash, generate_session_id};
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl AuthService for AppState {
    async fn sign_up(
        &self,
        request: Request<SignUpRequest>,
    ) -> Result<Response<SignUpResponse>, Status> {
        let req = request.into_inner();
        let username = req.username;
        let password = req.password;

        let password_hash = generate_password_hash(&password, &self.pass_pepper);
        if let Err(_) = password_hash {
            return Err(Status::internal("Password hashing failed"));
        }

        let created_user = create_user(&self.db_pool, &username, &password_hash.unwrap());
        if let Err(_) = created_user {
            return Err(Status::internal("User already exists"));
        }

        let user_id = created_user.unwrap();
        let session_id = generate_session_id(user_id.id, &self.secret_key);
        if let Err(_) = session_id {
            return Err(Status::internal("Session ID generation error"));
        }

        let sid = session_id.unwrap();

        match save_session_id(&self.db_pool, user_id.id, &sid) {
            Ok(_) => Ok(Response::new(SignUpResponse { session_id: sid })),
            Err(_) => Err(Status::internal("Could not sign up")),
        }
    }

    async fn sign_in(
        &self,
        _request: Request<SignInRequest>,
    ) -> Result<Response<SignInResponse>, Status> {
        todo!()
    }

    async fn sign_out(
        &self,
        _request: Request<SignOutRequest>,
    ) -> Result<Response<SignOutResponse>, Status> {
        todo!()
    }
}
