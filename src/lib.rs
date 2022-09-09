#[macro_use] extern crate custom_derive;
#[macro_use] extern crate newtype_derive;


pub mod librr;
pub mod record;
pub mod binary_interface;
pub mod replay;

#[allow(warnings)]
pub mod bindgen;


pub use librr::*;
pub use record::*;
pub use binary_interface::*;
pub use replay::*;
pub use bindgen::*;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn simple_test_main(){
        assert_eq!(5,1+4);
    }

}
