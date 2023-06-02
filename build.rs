fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        // .out_dir("src/school_server")
        .compile(&["proto/school.proto"], &["proto"])?;
    Ok(())
}
