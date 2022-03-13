extern crate cmake;
use cmake::Config;

fn main() {


    let dst = Config::new("librr").build();       
    
    println!("cargo:rerun-if-changed=librr/src/*");
    cxx_build::bridge("src/main.rs")
        .file("src/librr.cpp")
        .include("src")
        .include("librr/src")
        .include(format!("{}/build", dst.display()))
        .flag_if_supported("-std=c++14")
        .compile("librr-rs");

    
    println!("cargo:rustc-link-search=native={}/bin", dst.display());
    println!("cargo:rustc-link-search=native={}/lib/rr", dst.display());
    println!("cargo:rustc-link-lib=rrpreload");   
    println!("cargo:rustc-link-lib=rrpage");   
    println!("cargo:rustc-link-lib=rr");   
    println!("cargo:rustc-link-lib=rraudit");   
}
