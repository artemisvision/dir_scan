extern crate cbindgen;

use std::env;
use std::path::PathBuf;
use cbindgen::Config;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let output_file = target_dir()
        .join(format!("{}.hpp", package_name))
        .display()
        .to_string();

    println!("AAAA--- {:?}", crate_dir);
    match cbindgen::generate(&crate_dir) {
        Ok(e) => {e.write_to_file(&output_file);}
        Err(err) => {
            println!("Something went wrong {}", err);
        }
    }
}

fn target_dir() -> PathBuf {
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(target)
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
    }
}
