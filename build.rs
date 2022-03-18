extern crate cmake;
use cmake::Config;

fn main() {


    let dst = Config::new("librr").build();       
    
    //println!("cargo:rerun-if-changed=librr/src/*");

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
