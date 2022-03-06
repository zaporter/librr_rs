use cxx::UniquePtr;

use std::usize;


#[cxx::bridge]
mod ffi {
    #[namespace = "rr"]
    unsafe extern "C++" {
        include!("librr-rs/src/librr.hpp");
        fn page_size() -> usize;
    }
}

fn main() {
    //let mut printer = ffi::new_zprinter();
    
    //printer.print("Hello C plus plus".to_owned());
    println!("Hello, world! {}", ffi::page_size());
}

