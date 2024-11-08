use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let generated_dir = "../target/generated";

    if Path::new(generated_dir).exists() {
        fs::remove_dir_all(generated_dir)?;
    }

    fs::create_dir_all(generated_dir)?;

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(generated_dir)
        .compile_protos(&["../proto/news.proto"], &["../proto"])?;

    let out_dir = format!("../{}", generated_dir);
    println!("cargo:rustc-env=PROTO_OUT_DIR={}", out_dir);
    println!("cargo:rerun-if-changed=../proto/news.proto");

    Ok(())
}
