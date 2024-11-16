use std::collections::HashSet;
use crate::consts::{AUTHORIZE_HEADER, REQUEST_PATH};
use tonic::metadata::{Ascii, MetadataValue};
use tonic::service::Interceptor;
use tonic::{Request, Status};
use crate::settings::Settings;

#[derive(Clone)]
pub struct AuthInterceptor {
    valid_token: String,
    secure_routes: HashSet<String>
}

impl AuthInterceptor {
    pub fn new(settings: &Settings) -> Self {
        Self {
            valid_token: settings.auth.valid_token.clone(),
            secure_routes: settings.auth.get_secure_routes(),
        }
    }

    fn token_is_valid(&self, token: Option<&MetadataValue<Ascii>>) -> bool {
        token
            .and_then(|t| t.to_str().ok())
            .map(|t| t == self.valid_token)
            .unwrap_or(false)
    }
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        let metadata = request.metadata();

        let request_path = metadata
            .get(REQUEST_PATH)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| Status::unauthenticated("No RPC metadata or invalid path format"))?;

        if self.secure_routes.contains(request_path)
            && !self.token_is_valid(metadata.get(AUTHORIZE_HEADER))
        {
            return Err(Status::unauthenticated("Invalid token"));
        }

        Ok(request)
    }
}
