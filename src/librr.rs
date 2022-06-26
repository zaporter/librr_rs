#[cxx::bridge]
pub mod librrffi {
    #[namespace = "rr"]
    unsafe extern "C++" {
        include!("librr-rs/src/librr.hpp");
        pub fn page_size() -> usize;
        pub fn raise_resource_limits();
        pub fn assert_prerequisites(use_syscall_buffer : bool);
        pub fn test_replay_cpp();
    }

}
pub use librrffi::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn librr_replay_cpp_test(){
        assert!(page_size()>0);
        assert_eq!(5,1+4);
        raise_resource_limits();
        test_replay_cpp();
    }
}
