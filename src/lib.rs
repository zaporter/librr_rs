//TODO:
//Use throw catch to return Result<> types instead of c style return codes.

mod librr;
mod binary_interface;

#[allow(warnings)]
mod bindgen;


pub use librr::*;
pub use binary_interface::*;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn simple_test_main(){
        assert_eq!(5,1+4);
    }

}
