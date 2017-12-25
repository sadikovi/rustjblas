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

    // libraries to link
    let libraries = vec![
        ("gfortran", find_lib_path("libgfortran")),
        ("blas", find_lib_path("libblas")),
        ("lapack", find_lib_path("liblapack"))
    ];

    // compile cpp bindings into static lib which will be linked when we build library
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

    // add dynamic libraries to cargo build
    for (name, path) in libraries {
        println!("cargo:rustc-link-lib=dylib={}", name);
        println!("cargo:rustc-link-search=native={}", path);
    }
}

fn find_lib_path(lib_name: &str) -> String {
    match pkg_config::find_library(lib_name) {
        Ok(lib) => {
            for path in lib.include_paths.iter() {
                return format!("{}", path.parent().unwrap().display());
            }
        },
        Err(_) => {
            // cannot find library in pkg-config, search manually in /usr space
            for entry in WalkDir::new("/usr").into_iter().filter_map(|e| e.ok()) {
                if entry.file_name().to_str().map(|s| s.starts_with(lib_name)).unwrap_or(false) {
                    return format!("{}", entry.path().parent().unwrap().display());
                }
            }
        },
    }
    panic!("Failed to find path for library {}", lib_name);
}
