use axum::http::StatusCode;
use log::{error, log};
use redis::RedisError;
use crate::err::SchoolErr::{NotFound, RedisErr};

#[derive(Debug)]
pub enum SchoolErr {
    RedisErr(RedisError),
    SerdeJsonErr(serde_json::Error),
    NotFound,
}

impl From<RedisError> for SchoolErr {
    fn from(err: RedisError) -> Self {
        error!("redis error: {}", err);
        RedisErr(err)
    }
}

impl From<SchoolErr> for StatusCode {
    fn from(err: SchoolErr) -> Self {
        error!("school error: {:?}", err);
        return match err {
            RedisErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            SchoolErr::SerdeJsonErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            NotFound => StatusCode::NOT_FOUND,
        };
    }
}

impl From<serde_json::Error> for SchoolErr {
    fn from(err: serde_json::Error) -> Self {
        error!("serde_json error: {}", err);
        SchoolErr::SerdeJsonErr(err)
    }
}