extern crate bindgen;
extern crate cmake;

use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() -> miette::Result<()> {
    // println!("cargo:rerun-if-changed=librr/src/*.h");
    // println!("cargo:rerun-if-changed=librr/src/*.cc");
    // println!("cargo:rerun-if-changed=src/*.hpp");
    // println!("cargo:rerun-if-changed=src/*.cpp");
    // println!("cargo:rerun-if-changed=src/*.rs");
    // println!("cargo:rerun-if-changed=build.rs");

    let dst = Config::new("librr").build();

    //let path = std::path::PathBuf::from("src"); // include path
    //let path2  = std::path::PathBuf::from(format!("{}/build",dst.display()));
    //let mut b = autocxx_build::Builder::new("src/binary_interface.rs", &[&path,&path2, &dst]).build()?;
    //b
    //    .include("src")
    //    .include("librr/src")
    //    .include(format!("{}/build", dst.display()))
    //    .flag_if_supported("-std=c++14")
    //    .compile("autocxx-demo"); // arbitrary library name, pick anything
    //                           //
    {
        let bindings = bindgen::Builder::default()
            .header("librr/src/GdbConnection.h")
            .allowlist_type("rr::GdbRegisterValue")
            .allowlist_type("rr::GdbThreadId")
            .allowlist_type("rr::GdbContAction")
            .rustified_enum("rr::GdbRegister")
            .rustified_enum("rr::GdbActionType")
            .generate_comments(true)
            .derive_debug(true)
            .derive_default(true)
            .translate_enum_integer_types(true)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .clang_arg("-xc++")
            .clang_arg("-std=c++14")
            .clang_arg(format!("-I{}/build", dst.display()))
            .generate()
            .or(Err("Unable to generate bindings for GdbConnection"))
            .unwrap();
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_dir.join("gdbconnection-bindings.rs"))
            .expect("Couldn't write bindings!");
    }
    {
        let bindings = bindgen::Builder::default()
            .header("librr/src/TaskishUid.h")
            .allowlist_type("rr::TaskUid")
            .generate_comments(true)
            .derive_default(true)
            .translate_enum_integer_types(true)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .clang_arg("-xc++")
            .clang_arg("-std=c++14")
            .clang_arg(format!("-I{}/build", dst.display()))
            .generate()
            .or(Err("Unable to generate bindings for TaskishUid"))
            .unwrap();
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_dir.join("taskishuid-bindings.rs"))
            .expect("Couldn't write bindings!");
    }

    cxx_build::bridge("src/librr.rs")
        .file("src/librr.cpp")
        .include("src")
        .include("librr/src")
        .include(format!("{}/build", dst.display()))
        .flag_if_supported("-std=c++14")
        .compile("librr-rs-librr");

    cxx_build::bridge("src/record.rs")
        .file("src/record.cpp")
        .include("src")
        .include("librr/src")
        .include(format!("{}/build", dst.display()))
        .flag_if_supported("-std=c++14")
        .compile("librr-rs-record");

    cxx_build::bridge("src/replay.rs")
        .file("src/replay.cpp")
        .include("src")
        .include("librr/src")
        .include(format!("{}/build", dst.display()))
        .flag_if_supported("-std=c++14")
        .compile("librr-rs-replay");

    cxx_build::bridge("src/binary_interface.rs")
        .file("src/binary_interface.cpp")
        .include("src")
        .include("librr/src")
        .include(format!("{}/build", dst.display()))
        .flag_if_supported("-std=c++14")
        .compile("librr-rs-binary-interface");

    println!("cargo:rustc-link-search=native={}/bin", dst.display());
    println!("cargo:rustc-link-search=native={}/lib/rr", dst.display());
    println!("cargo:rustc-link-lib=rrpreload");
    println!("cargo:rustc-link-lib=rrpage");
    println!("cargo:rustc-link-lib=rr");
    println!("cargo:rustc-link-lib=rraudit");
    Ok(())
}
