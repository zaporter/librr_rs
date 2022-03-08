use cxx::UniquePtr;

use std::usize;

//pub mod simple;

#[cxx::bridge]
mod ffi {

    enum SyscallBuffering {
        ENABLE_SYSCALL_BUF,
        DISABLE_SYSCALL_BUF,
    }
    struct RecordFlags {
        extra_env : Vec<String>,
        max_ticks : i64,
        ignore_sig : i32,
        continue_through_sig : i32,


    }
    #[namespace = "rr"]
    unsafe extern "C++" {
        include!("librr-rs/src/librr.hpp");
        fn page_size() -> usize;
    }
    #[namespace = "rr"]
    unsafe extern "C++" {
        include!("librr-rs/src/librr.hpp");
        type Flags;
    }
    struct Zags {
        date : i32,
        count : f32,
    }

    unsafe extern "C++" {
        include!("librr-rs/src/librr.hpp");
        type Zags;
        fn createZags() -> Zags;
        fn printZags(zags : Zags);
        fn testCPPFunction() -> i32;
    }
}

fn main() {
    //let mut printer = ffi::new_zprinter();
    ffi::testCPPFunction();
    let mut zags = ffi::createZags();
    println!("Date: {}, count: {}", zags.date, zags.count);
    zags.date = 35;
    ffi::printZags(zags);
    //printer.print("Hello C plus plus".to_owned());
    println!("Hello, world! {}", ffi::page_size());
}

