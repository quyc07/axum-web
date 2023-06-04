pub mod db;
pub mod err;
pub mod school;
pub mod sea;
pub mod templates;
pub mod async_db;

pub mod school_proto {
    tonic::include_proto!("school_proto");
}
