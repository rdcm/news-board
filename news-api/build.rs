use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = "../target/generated";

    if Path::new(out_dir).exists() {
        fs::remove_dir_all(out_dir)?;
    }

    fs::create_dir_all(out_dir)?;

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(out_dir)
        .compile_protos(&["../proto/news.proto"], &["../proto"])?;

    println!("cargo:rustc-env=PROTO_OUT_DIR={}", format!("../{}", out_dir));
    println!("cargo:rerun-if-changed=../proto/news.proto");

    Ok(())
}