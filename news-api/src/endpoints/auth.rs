use crate::app_state::AppState;
use crate::auth_generated::auth_service_server::AuthService;
use crate::auth_generated::{
    SignInRequest, SignInResponse, SignOutRequest, SignOutResponse, SignUpRequest, SignUpResponse,
};
use crate::infrastructure::{create_user, get_user_by_username, save_session_id};
use crate::utils::{generate_password_hash, generate_session_id, verify_password};
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
        if password_hash.is_err() {
            return Err(Status::internal("Password hashing failed"));
        }

        let hash = password_hash.unwrap();
        let created_user = create_user(&self.db_pool, &username, &hash.value, &hash.salt);
        if created_user.is_err() {
            return Err(Status::internal("User already exists"));
        }

        let user_id = created_user.unwrap();
        let session_id = generate_session_id(user_id.id, &self.secret_key);
        if session_id.is_err() {
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
        request: Request<SignInRequest>,
    ) -> Result<Response<SignInResponse>, Status> {
        let req = request.into_inner();
        let username = req.username;
        let password = req.password;

        let user_entry = get_user_by_username(&self.db_pool, &username);
        if user_entry.is_err() {
            return Err(Status::not_found("Username not found"));
        }

        let user = user_entry.unwrap();
        let password_verification = verify_password(
            &password,
            &user.password_hash,
            &user.salt,
            &self.pass_pepper,
        );
        if password_verification.is_err() {
            return Err(Status::invalid_argument("Invalid password"));
        }

        let session_id = generate_session_id(user.id, &self.secret_key);
        if session_id.is_err() {
            return Err(Status::internal("Session ID generation error"));
        }

        let sid = session_id.unwrap();
        match save_session_id(&self.db_pool, user.id, &sid) {
            Ok(_) => Ok(Response::new(SignInResponse { session_id: sid })),
            Err(_) => Err(Status::internal("Could not sign in")),
        }
    }

    async fn sign_out(
        &self,
        _request: Request<SignOutRequest>,
    ) -> Result<Response<SignOutResponse>, Status> {
        todo!()
    }
}
