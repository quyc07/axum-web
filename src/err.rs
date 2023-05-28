use axum::http::StatusCode;
use redis::RedisError;
use crate::err::SchoolErr::{NotFound, RedisErr};

#[derive(Debug)]
pub enum SchoolErr {
    RedisErr(RedisError),
    NotFound,
}

impl From<RedisError> for SchoolErr {
    fn from(err: RedisError) -> Self {
        RedisErr(err)
    }
}

impl From<SchoolErr> for StatusCode {
    fn from(err: SchoolErr) -> Self {
        return match err {
            RedisErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            NotFound => StatusCode::NOT_FOUND,
        };
    }
}