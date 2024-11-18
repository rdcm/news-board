pub const REQUEST_PATH_HEADER: &str = "x-request-path";
pub const AUTHORIZE_HEADER: &str = "Authorize";

#[derive(Clone)]
pub struct SessionId {
    pub value: String,
}
#[derive(Clone)]
pub struct UserId {
    pub value: i32,
}
