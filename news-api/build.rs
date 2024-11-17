use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let generated_dir_path = "../target/generated";

    if Path::new(generated_dir_path).exists() {
        fs::remove_dir_all(generated_dir_path)?;
    }

    fs::create_dir_all(generated_dir_path)?;

    let proto_paths = fs::read_dir("../proto/")
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect::<Vec<_>>();

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(generated_dir_path)
        .compile_protos(&proto_paths, &["../proto"])?;

    let out_dir = format!("../{}", generated_dir_path);
    println!("cargo:rustc-env=PROTO_OUT_DIR={}", out_dir);

    Ok(())
}
