#[path = "../../target/generated"]
pub mod news {
    include!(concat!(env!("PROTO_OUT_DIR"), "/news.rs"));
}