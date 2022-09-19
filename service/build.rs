fn main() -> Result<(), Box<dyn std::error::Error>> {
    const PROTO_PATH: &str = "../file_service.proto";
    tonic_build::compile_protos(PROTO_PATH)?;
    Ok( () )
}