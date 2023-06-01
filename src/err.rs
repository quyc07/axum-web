use crate::err::SchoolErr::{NotFound, RedisErr};
use axum::http::StatusCode;
use log::{error, log};
use redis::RedisError;

#[derive(Debug)]
pub enum SchoolErr {
    RedisErr(RedisError),
    SerdeJsonErr(serde_json::Error),
    MysqlErr(mysql::Error),
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
            SchoolErr::MysqlErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
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

impl From<mysql::Error> for SchoolErr {
    fn from(error: mysql::Error) -> Self {
        error!("mysql error:{}", error);
        SchoolErr::MysqlErr(error)
    }
}
