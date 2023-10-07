use std::{env::var, fs, io, io::prelude::*, io::ErrorKind::AlreadyExists, path::PathBuf};

const FAASM_VENDOR_FOLDER: &str = "vendor/faasm/";
const LIBFAASM_PATH: &str = "cpp/libfaasm";
const FAASM_VERSION: &str = env!("CARGO_PKG_VERSION");

// Generates a wrapper header pointing to the FaasmRelease sysroot
fn generate_header(root_dir: &str) -> io::Result<String> {
    let header_file = format!("{root_dir}/wrapper-{FAASM_VERSION}.h");
    match fs::File::create(&header_file) {
        Ok(mut header_handler) => {
            let header_content = format!(
                "\
                #include \"{LIBFAASM_PATH}/faasm/host_interface.h\"\n\
                #include \"{LIBFAASM_PATH}/faasm/faasm.h\"\n\
            "
            );
            header_handler.write_all(&header_content.into_bytes())?;
            Ok(header_file)
        }
        Err(err) if err.kind() == AlreadyExists => Ok(header_file),
        Err(err) => Err(err),
    }
}

// TODO - implement bindings using bindgen. I spent several hours with it but
// can still only generate blank outputs using the library
fn generate_bindings(_wrapper: &str, output_file: &PathBuf) -> io::Result<()> {
    // Warn that dynamic binding generation is not implemented and needs to be ran
    // a-priori of build from the cmd line with bindgen.
    println!("cargo:warning=Using manually generated bindings");

    // Bindings file maintained manually in this repo
    let vendor_folder = PathBuf::from(FAASM_VENDOR_FOLDER);
    // Output file where we want to link things
    let manual_gen_file = vendor_folder.join("bindings.rs");

    // Location that can be included in the libray code
    fs::copy(manual_gen_file, output_file).unwrap();

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target = var("TARGET").unwrap();

    if !target.starts_with("wasm32") {
        panic!("Can only build faasm-sys for a WASM target");
    }

    // Location of the binding file that will be included in the library
    let binding_file = PathBuf::from(var("OUT_DIR").unwrap()).join("bindings.rs");

    let header = generate_header(FAASM_VENDOR_FOLDER)?;

    // Rerun if the wrapper is changed (more relevant for dev mode)
    println!("cargo:rerun-if-changed={header}");

    // TODO - this only copies the manually generated bindings
    generate_bindings(&header, &binding_file)?;

/*    let source_files: Vec<_> = vec![
        "compare.cpp",
        "core.cpp",
        "files.cpp",
        "input.cpp",
        "print.cpp",
        "random.cpp",
        "state.cpp",
        "time.cpp",
        "zygote.cpp",
    ]
    .into_iter()
    .map(|f| format!("{LIBFAASM_PATH}/{f}"))
    .collect();

    cc::Build::new()
        .cpp(true)
        .cpp_set_stdlib("c++")
        .std("c++17")
        .include(LIBFAASM_PATH)
        .flag("--sysroot=/usr/share/wasi-sysroot")
        .files(source_files)
        .compile("faasm");

    /* WASM support in CMake seems broken...

    let dst = cmake::Config::new("cpp/libfaasm")
                .define("CMAKE_HOST_SYSTEM_NAME", "Linux")
                .define("CMAKE_SYSTEM_NAME", "Wasm")
                .build();
    */

    //println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=faasm");*/

    Ok(())
}
