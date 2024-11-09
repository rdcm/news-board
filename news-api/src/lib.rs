pub mod endpoints;
pub mod infrastructure;
pub mod mappers;
pub mod services;
pub mod settings;
pub mod utils;

#[path = "../../target/generated"]
pub mod news {
    include!(concat!(env!("PROTO_OUT_DIR"), "/news.rs"));
}
