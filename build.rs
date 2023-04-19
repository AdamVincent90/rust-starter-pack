// TODO - not final, this will be moved.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("scaffold/protobuf/helloworld.proto")?;
    Ok(())
}
