use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = PathBuf::from(&manifest_dir);

    let server_package = manifest_path.join("server/js/package.json");
    let server_mjs = manifest_path.join("server/js/server.mjs");
    let client_package = manifest_path.join("client/js/package.json");
    let client_mjs = manifest_path.join("client/js/client.mjs");

    println!("cargo:rerun-if-changed=server/js/package.json");
    println!("cargo:rerun-if-changed=server/js/server.mjs");
    println!("cargo:rerun-if-changed=client/js/package.json");
    println!("cargo:rerun-if-changed=client/js/client.mjs");

    let out_dir = manifest_path.join("src/embedded");
    fs::create_dir_all(&out_dir).expect("Failed to create embedded directory");

    fs::copy(&server_package, out_dir.join("package.json")).expect("Failed to copy package.json");
    fs::copy(&server_mjs, out_dir.join("server.mjs")).expect("Failed to copy server.mjs");
    fs::copy(&client_package, out_dir.join("package.json")).expect("Failed to copy package.json");
    fs::copy(&client_mjs, out_dir.join("client.mjs")).expect("Failed to copy client.mjs");
}
