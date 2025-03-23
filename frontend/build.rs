use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("assets");
    fs::create_dir_all(&dest_path).unwrap();

    // Copier le logo
    fs::copy(
        "public/assets/logo.svg",
        dest_path.join("logo.svg"),
    ).unwrap();

    // Copier les styles
    fs::copy(
        "src/styles.css",
        dest_path.join("styles.css"),
    ).unwrap();
} 