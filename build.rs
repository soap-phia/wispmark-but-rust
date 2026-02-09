use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=../server/js/package.json");
    println!("cargo:rerun-if-changed=../server/js/server.mjs");
    println!("cargo:rerun-if-changed=../client/js/client.mjs");

    let out_dir = Path::new("src/embedded");
    fs::create_dir_all(out_dir).expect("Failed to create embedded directory");

    fs::copy("../server/js/package.json", out_dir.join("package.json"))
        .expect("Failed to copy package.json");
    fs::copy("../server/js/server.mjs", out_dir.join("server.mjs"))
        .expect("Failed to copy server.mjs");
    fs::copy("../client/js/client.mjs", out_dir.join("client.mjs"))
        .expect("Failed to copy client.mjs");
}
