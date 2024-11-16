use crate::consts::REQUEST_PATH_HEADER;
use std::task::{Context, Poll};
use tonic::codegen::http::HeaderValue;
use tonic::codegen::{http, Service};
use tower::Layer;

#[derive(Debug, Clone, Default)]
pub struct ReflectionMiddlewareLayer {}

impl<S> Layer<S> for ReflectionMiddlewareLayer {
    type Service = ReflectionMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        ReflectionMiddleware { inner: service }
    }
}

#[derive(Debug, Clone)]
pub struct ReflectionMiddleware<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<http::Request<ReqBody>> for ReflectionMiddleware<S>
where
    S: Service<http::Request<ReqBody>, Response = http::Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: http::Request<ReqBody>) -> Self::Future {
        let path = req.uri().path().to_owned();
        req.headers_mut().insert(
            REQUEST_PATH_HEADER,
            HeaderValue::from_str(&path).unwrap(),
        );

        self.inner.call(req)
    }
}
