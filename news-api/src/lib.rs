#[path = "auth/auth_interceptor.rs"]
pub mod auth_interceptor;
#[path = "auth/consts.rs"]
pub mod consts;
#[path = "endpoints.rs"]
pub mod endpoints;
#[path = "infrastructure.rs"]
pub mod infrastructure;
#[path = "mappers.rs"]
pub mod mappers;
#[path = "auth/reflection_middleware.rs"]
pub mod reflection_middleware;
#[path = "services.rs"]
pub mod services;
#[path = "settings.rs"]
pub mod settings;
#[path = "../../target/generated"]
pub mod news {
    include!(concat!(env!("PROTO_OUT_DIR"), "/news.rs"));
}
