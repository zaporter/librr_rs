
#[cxx::bridge]
pub mod ffi {
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
