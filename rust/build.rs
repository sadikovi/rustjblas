extern crate cc;
extern crate pkg_config;
extern crate walkdir;

use std::env;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    // set by cargo
    let target = env::var("TARGET").unwrap();
    let java_home = env::var("JAVA_HOME").expect("JAVA_HOME variable is not set");

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
                }
            }
        },
    }

    // include openblas lib, this is pulled as external crate, we just need to link it
    // ignore link on osx, otherwise it fails executing blas functions
    if !target.contains("apple") {
        println!("cargo:rustc-link-lib=static=openblas");
    }

    // Compile cpp bindings into static lib which will be linked when we build library
    let java_include = Path::new(&java_home).join("include");
    let platform_suffix = if target.contains("apple") { "darwin" } else { "linux" };
    let java_include_platform = Path::new(&java_home).join("include").join(platform_suffix);

    // this will include cpp as static lib
    cc::Build::new()
        .cpp(true)
        .file("format/jblas_format.cpp")
        .include(java_include)
        .include(java_include_platform)
        .compile("libformat.a");
}
