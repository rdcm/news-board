use crate::app_state::AppState;
use crate::consts::{SessionId, UserId, AUTHORIZE_HEADER, REQUEST_PATH_HEADER};
use crate::infrastructure::get_session_by_id;
use tonic::service::Interceptor;
use tonic::{Request, Status};

#[derive(Clone)]
pub struct AuthInterceptor {
    app_state: AppState,
}

impl AuthInterceptor {
    pub fn new(app_state: AppState) -> Self {
        Self { app_state }
    }
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let metadata = request.metadata();

        let request_path = metadata
            .get(REQUEST_PATH_HEADER)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| Status::unauthenticated("No RPC metadata or invalid path format"))?;

        if self
            .app_state
            .settings
            .auth
            .secure_routes
            .contains(request_path)
        {
            let session_id = metadata
                .get(AUTHORIZE_HEADER)
                .ok_or(Status::unauthenticated("Invalid token"))?
                .to_str()
                .map_err(|_| Status::unauthenticated("Invalid token"))?;

            let user_id = get_session_by_id(&self.app_state.db_pool, session_id)
                .map_err(|_| Status::unauthenticated("No such session"))?;

            let sid_string = session_id.to_string();
            let extensions = request.extensions_mut();
            extensions.insert(UserId { value: user_id.id });
            extensions.insert(SessionId { value: sid_string });
        }

        Ok(request)
    }
}
