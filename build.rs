fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/helloworld.proto")?;
    tonic_build::configure()
        .out_dir("src/db")
        .compile(&["proto/school.proto"], &["proto"])?;
    Ok(())
}
