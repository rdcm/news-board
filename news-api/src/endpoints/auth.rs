use crate::app_state::AppState;
use crate::auth_generated::auth_service_server::AuthService;
use crate::auth_generated::*;
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

        let password_hash = generate_password_hash(&password, &self.settings.auth.pass_pepper)
            .map_err(|_| Status::unauthenticated("Password hashing failed"))?;

        let created_user = create_user(
            &self.db_pool,
            &username,
            &password_hash.value,
            &password_hash.salt,
        )
        .map_err(|_| Status::unauthenticated("Could not create user"))?;

        let session_id = generate_session_id(created_user.id, &self.settings.auth.secret_key)
            .map_err(|_| Status::unauthenticated("Session ID generation error"))?;

        save_session_id(&self.db_pool, created_user.id, &session_id)
            .map_err(|_| Status::unauthenticated("Could not sign up"))?;

        Ok(Response::new(SignUpResponse { session_id }))
    }

    async fn sign_in(
        &self,
        request: Request<SignInRequest>,
    ) -> Result<Response<SignInResponse>, Status> {
        let req = request.into_inner();
        let username = req.username;
        let password = req.password;

        let user = get_user_by_username(&self.db_pool, &username)
            .map_err(|_| Status::unauthenticated("User not found"))?;

        verify_password(
            &password,
            &user.password_hash,
            &user.salt,
            &self.settings.auth.pass_pepper,
        )
        .map_err(|_| Status::unauthenticated("Invalid password"))?;

        let session_id = generate_session_id(user.id, &self.settings.auth.secret_key)
            .map_err(|_| Status::unauthenticated("Session ID generation error"))?;

        save_session_id(&self.db_pool, user.id, &session_id)
            .map_err(|_| Status::unauthenticated("Could not sign in"))?;

        Ok(Response::new(SignInResponse { session_id }))
    }

    async fn sign_out(
        &self,
        _request: Request<SignOutRequest>,
    ) -> Result<Response<SignOutResponse>, Status> {
        todo!()
    }
}
