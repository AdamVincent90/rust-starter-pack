fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("scaffold/proto/user.proto")?;
    Ok(())
}
