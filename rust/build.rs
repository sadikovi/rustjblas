extern crate pkg_config;
extern crate walkdir;

use std::env;
use walkdir::WalkDir;

fn main() {
    // include gfortran library, if it is accessible through pkg-config, otherwise search /usr
    // space to find the library
    match pkg_config::find_library("libgfortran") {
        Ok(lib) => {
            for path in lib.include_paths.iter() {
                println!("cargo:rustc-link-search=native={}", path.parent().unwrap().display());
            }
        },
        Err(_) => {
            // cannot find libgfortran in pkg-config, search manually in /usr space
            for entry in WalkDir::new("/usr").into_iter().filter_map(|e| e.ok()) {
                if entry.file_name().to_str().map(|s| s == "libgfortran.a").unwrap_or(false) {
                    println!("cargo:rustc-link-search=native={}",
                        entry.path().parent().unwrap().display());
                    break;
                }
            }
        },
    }

    // linking is used for testing only, therefore it might be missing certain optimizations;
    // we build it correctly with optimizations when running `make build` or `make release`.

    // link propack lib
    let mut propack_path = env::current_dir().unwrap();
    propack_path.pop(); // go level up to the project directory
    propack_path.push("propack");
    propack_path.push("target");

    println!("cargo:rustc-link-lib=dylib=openblas");
    // include propack library for dlansvd functions
    println!("cargo:rustc-link-search=native={}", propack_path.as_path().to_str().unwrap());
    println!("cargo:rustc-link-lib=static=propack");
}
