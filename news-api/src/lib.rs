pub mod endpoints;
pub mod infrastructure;
pub mod services;
pub mod settings;

#[path = "../../target/generated"]
pub mod news {
    include!(concat!(env!("PROTO_OUT_DIR"), "/news.rs"));
}
