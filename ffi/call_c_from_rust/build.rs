use core::panic;
use std::env;
use std::fs;
use std::process::Command;

struct CLibBuildOptions {
    cmake_generator: String,
    library_name: String,
    library_path: String,
    build_path: String,
}

impl Default for CLibBuildOptions {
    fn default() -> Self {
        Self {
            cmake_generator: Default::default(),
            library_name: Default::default(),
            library_path: Default::default(),
            build_path: Default::default(),
        }
    }
}

/// https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
fn get_c_lib_build_options() -> CLibBuildOptions {
    let mut build_options = CLibBuildOptions::default();

    let target = env::var("TARGET").unwrap();
    println!("cargo:warning=Target: {}", target);
    if env::var("CARGO_CFG_UNIX").is_ok() {
        // Unix toolchains
        build_options.cmake_generator = "Ninja Multi-Config".to_string();
    } else {
        // Windows toolchains
        if target == "x86_64-pc-windows-gnu" {
            build_options.cmake_generator = "Ninja Multi-Config".to_string();
        } else if target == "x86_64-pc-windows-msvc" {
            build_options.cmake_generator = "Visual Studio 16 2019".to_string();
        } else {
            panic!("Target not supported: {}", target);
        }
    };

    // Common
    build_options.library_name = "c_lib".to_string();
    build_options.build_path = format!("_build_{}", target);
    build_options.library_path = format!("{}/Release", build_options.build_path);
    build_options
}

/// https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn main() {
    // Get generator and lib_path dependending on "target" to be built
    let build_options = get_c_lib_build_options();

    // Generate
    let _output = Command::new("cmake")
        .args([
            "-G",
            &build_options.cmake_generator,
            "-B",
            &build_options.build_path,
            "-S",
            "c_lib",
            "-DCMAKE_BUILD_TYPE:STRING=Release",
        ])
        .output()
        .expect("Failed to execute cmake command");

    // Build
    let _output = Command::new("cmake")
        .args(["--build", &build_options.build_path, "--config", "Release"])
        .output()
        .expect("Failed to build cmake command");

    // Print information
    println!(
        "cargo:warning=Current Directory: {:?}",
        env::current_dir().unwrap()
    );
    let dir_iter = fs::read_dir(build_options.library_path.clone());
    if dir_iter.is_ok() {
        dir_iter.unwrap().for_each(|x| {
            if x.is_ok() {
                let dir_entry = x.unwrap();
                println!("cargo:warning=Dir Entry: {:?}", dir_entry);
            }
        });
    }

    // Link to project
    println!("cargo:rustc-link-search={}", build_options.library_path);
    println!("cargo:rustc-link-lib={}", build_options.library_name);
}
