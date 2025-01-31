use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("service_descriptor.bin"))
        .compile_protos(&["proto/service.proto"], &["proto"])?;
    Ok(())
}
