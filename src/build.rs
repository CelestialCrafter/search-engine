use std::io::Error;

fn main() -> Result<(), Error> {
    prost_build::compile_protos(&["src/protos/crawled.proto"], &["src/protos"])?;
    Ok(())
}
