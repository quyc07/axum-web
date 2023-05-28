use redis::RedisError;

#[derive(Debug)]
pub enum SchoolErr {
    RedisErr(RedisError),
    NotFound,
}

impl From<RedisError> for SchoolErr {
    fn from(err: RedisError) -> Self {
        SchoolErr::RedisErr(err)
    }
}