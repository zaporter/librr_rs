#[macro_use] extern crate custom_derive;
#[macro_use] extern crate newtype_derive;


mod librr;
mod record;
mod binary_interface;
mod replay;

#[allow(warnings)]
mod bindgen;


pub use librr::*;
pub use record::*;
pub use binary_interface::*;
pub use replay::*;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn simple_test_main(){
        assert_eq!(5,1+4);
    }

}
