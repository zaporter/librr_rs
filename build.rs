extern crate cmake;
extern crate bindgen;

use std::env;
use std::path::PathBuf;
use cmake::Config;

fn main() {

    println!("cargo:rerun-if-changed=librr/src/*.h");
    println!("cargo:rerun-if-changed=librr/src/*.cc");
    println!("cargo:rerun-if-changed=src/*.hpp");
    println!("cargo:rerun-if-changed=src/*.cpp");
    println!("cargo:rerun-if-changed=build.rs");

    let dst = Config::new("librr").build();       
    {   
        let bindings = bindgen::Builder::default()
            .header("librr/src/GdbConnection.h")
            .allowlist_type("rr::GdbRegisterValue")
            .allowlist_type("rr::GdbThreadId")
            .newtype_enum("rr::GdbRegister")
            .generate_comments(true)
            .derive_default(true)
            .translate_enum_integer_types(true)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .clang_arg("-xc++")
            .clang_arg("-std=c++14")
            .clang_arg(format!("-I{}/build", dst.display()))
            .generate()
            .or(Err("Unable to generate bindings for GdbConnection")).unwrap();
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings.write_to_file(out_dir.join("gdbconnection-bindings.rs")).expect("Couldn't write bindings!");
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
            .or(Err("Unable to generate bindings for TaskishUid")).unwrap();
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings.write_to_file(out_dir.join("taskishuid-bindings.rs")).expect("Couldn't write bindings!");
    }
    cxx_build::bridge("src/librr.rs")
        .file("src/librr.cpp")
        .include("src")
        .include("librr/src")
        .include(format!("{}/build", dst.display()))
        .flag_if_supported("-std=c++14")
        .compile("librr-rs-librr");

    cxx_build::bridge("src/zags.rs")
        .file("src/zags.cpp")
        .include("src")
        //.include(format!("{}/build", dst.display()))
        .flag_if_supported("-std=c++14")
        .compile("librr-rs-zags");
    
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

    
    println!("cargo:rustc-link-search=native={}/bin", dst.display());
    println!("cargo:rustc-link-search=native={}/lib/rr", dst.display());
    println!("cargo:rustc-link-lib=rrpreload");   
    println!("cargo:rustc-link-lib=rrpage");   
    println!("cargo:rustc-link-lib=rr");   
    println!("cargo:rustc-link-lib=rraudit");   
}
