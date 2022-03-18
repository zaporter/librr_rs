#[cxx::bridge]
pub mod zagsffi {
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
pub use zagsffi::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zags_test(){
      assert_eq!(5,1+4);
      let zags = createZags();
      assert_eq!(zags.date, 10)
    }
}
