use std::env;
use std::path::PathBuf;

fn main() {
    // Re-run build if ffi.h changes
    println!("cargo:rerun-if-changed=ffi.h");

    // Probe fluidsynth using pkg-config to get include paths and link flags
    // Try to probe fluidsynth using pkg-config to get include paths, link paths and libs
    let mut include_args: Vec<String> = Vec::new();
    match pkg_config::Config::new().probe("fluidsynth") {
        Ok(lib) => {
            // Emit link flags for cargo
            for lib_path in lib.link_paths {
                println!("cargo:rustc-link-search=native={}", lib_path.display());
            }
            for lib_name in lib.libs {
                println!("cargo:rustc-link-lib={}", lib_name);
            }
            // Provide include paths to bindgen via -I flags
            for include_path in lib.include_paths {
                include_args.push(format!("-I{}", include_path.display()));
            }
        }
        Err(e) => {
            // If pkg-config fails, continue — bindgen may still work if system headers are in default include paths.
            eprintln!("warning: pkg-config probe for fluidsynth failed: {}", e);
        }
    }

    // Build bindgen
    let header = PathBuf::from("ffi.h");

    // Tell cargo to invalidate the built crate whenever the header changes
    println!("cargo:rerun-if-changed={}", header.display());

    // The bindgen::Builder requires libclang installed. The user must ensure libclang is available.
    let mut builder = bindgen::Builder::default()
        .header(header.to_string_lossy())
        .derive_debug(true)
        .derive_default(true)
        .generate_comments(true);

    for arg in include_args {
        builder = builder.clang_arg(arg);
    }

    let bindings = builder
        .generate()
        .expect("Unable to generate bindings for fluidsynth");

    // Write the bindings to the $OUT_DIR/ffi.rs
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("ffi.rs"))
        .expect("Couldn't write bindings!");
}
