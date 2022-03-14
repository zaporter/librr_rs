
#[cxx::bridge]
pub mod ffi {
    pub struct Zags {
        date : i32,
        count : f32,
    }

    unsafe extern "C++" {
        include!("librr-rs/src/zags.hpp");
        type Zags;
      //  type RecordingFlags;
        pub fn createZags() -> Zags;
        pub fn printZags(zags : Zags);
    }
}
