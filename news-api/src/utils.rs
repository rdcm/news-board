use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;

pub fn parse_timestamp(timestamp_str: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S%.6f").ok()
}
