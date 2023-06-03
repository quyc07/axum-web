pub mod db;
pub mod err;
pub mod school;
pub mod templates;

pub mod school_proto {
    tonic::include_proto!("school_proto");
}