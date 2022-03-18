#[cxx::bridge]
pub mod librrffi {
    #[namespace = "rr"]
    unsafe extern "C++" {
        include!("librr-rs/src/librr.hpp");
        pub fn page_size() -> usize;
        pub fn raise_resource_limits();
        pub fn assert_prerequisites(use_syscall_buffer : bool);
    }

}
pub use librrffi::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test(){
        assert!(page_size()>0);
        assert_eq!(5,1+4);
    }
}
