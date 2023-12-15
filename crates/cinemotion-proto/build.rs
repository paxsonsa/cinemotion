fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = prost_build::Config::new();
    // use Bytes type in protobuf messages to avoid additional
    // allocations of our raw yaml data
    config.bytes(["."]);
    // generate prost types for our service.proto
    let _ = config.compile_protos(&["src/defs/cinemotion.proto"], &["src/defs"])?;
    Ok(())
}
