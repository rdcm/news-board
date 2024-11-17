#[path = "app_state.rs"]
pub mod app_state;
#[path = "endpoints/auth.rs"]
pub mod auth;
#[path = "auth/auth_interceptor.rs"]
pub mod auth_interceptor;
#[path = "auth/consts.rs"]
pub mod consts;
#[path = "infrastructure.rs"]
pub mod infrastructure;
#[path = "mappers.rs"]
pub mod mappers;
#[path = "endpoints/news.rs"]
pub mod news;
#[path = "auth/reflection_middleware.rs"]
pub mod reflection_middleware;
#[path = "settings.rs"]
pub mod settings;
#[path = "utils.rs"]
mod utils;
#[path = "../../target/generated/news.rs"]
pub mod news_generated {
    include!(concat!(env!("PROTO_OUT_DIR"), "/news.rs"));
}
#[path = "../../target/generated/auth.rs"]
pub mod auth_generated {
    include!(concat!(env!("PROTO_OUT_DIR"), "/auth.rs"));
}
