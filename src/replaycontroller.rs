#[cxx::bridge]
pub mod replaycontrollerffi {
    unsafe extern "C++" {
        include!("librr-rs/src/replaycontroller.hpp");
        pub fn la_trip(val : i32) -> i32;
    }
}

pub use replaycontrollerffi::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trip_check(){
        assert_eq!(12, la_trip(4));
    }

}

