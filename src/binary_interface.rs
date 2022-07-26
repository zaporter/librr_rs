use cxx::UniquePtr;

use cxx::{type_id, ExternType};
unsafe impl ExternType for crate::bindgen::gdbconnection::rr_GdbThreadId {
    type Id = type_id!("rr::GdbThreadId");
    type Kind = cxx::kind::Trivial;
}
unsafe impl ExternType for crate::bindgen::gdbconnection::rr_GdbRegisterValue {
    type Id = type_id!("rr::GdbRegisterValue");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge(namespace="rr")]
pub mod binary_interface_ffi {


    unsafe extern "C++" {
        include!("librr-rs/src/binary_interface.hpp");
        type GdbThreadId = crate::bindgen::gdbconnection::rr_GdbThreadId;
        type GdbRegisterValue = crate::bindgen::gdbconnection::rr_GdbRegisterValue;
    }

    impl Vec<GdbThreadId>{}
    impl Vec<GdbRegisterValue>{}

    unsafe extern "C++" {

        type BinaryInterface;
        include!("librr-rs/src/binary_interface.hpp");
        // pub fn sayHi();
        pub fn new_binary_interface(goto_event: i64, trace_dir: String) -> UniquePtr<BinaryInterface>; 
        pub fn beta_test_me();
        pub fn gamma_test_me();
        pub fn delta_test_me();
        pub fn initialize(self: Pin<&mut BinaryInterface>) -> bool;
        pub fn current_frame_time(&self) -> i64;
        // pub fn get_thread_list(self: &BinaryInterface) -> Vec<GdbThreadId>;
        // pub fn get_current_thread(self: &BinaryInterface) -> GdbThreadId;
        // pub fn get_regs(self: &BinaryInterface, tid: i32) -> Vec<GdbRegisterValue>;
        // pub fn get_exec_file(self: &BinaryInterface, request_target : GdbThreadId) -> String;
    }
}
pub use binary_interface_ffi::*;

#[cfg(test)]
mod tests {
    use crate::raise_resource_limits;
    use serial_test::serial;

    use super::*;

    // #[test]
    // fn fuck_tsla(){
    //   assert_eq!(5,1+4);
    //   sayHi();
    //   let bin_interface = new_binary_interface("/home/zack/.local/share/rr/DateTester-41".to_owned());
    //   let threads = bin_interface.get_thread_list();
    //   dbg!(threads);
    //   let thread = bin_interface.get_current_thread();
    //   dbg!(thread);
    //   bin_interface.chicken_nuggs();
    // }
    #[test]
    #[serial]
    fn binary_interface_creation(){
      raise_resource_limits();
      let _bin_interface = new_binary_interface(0,"/home/zack/.local/share/rr/date_viewer-64".to_owned());
    }
    #[test]
    fn beta_test(){
      raise_resource_limits();
      beta_test_me();
    }
    #[test]
    fn gamma_test(){
      raise_resource_limits();
      gamma_test_me();
    }
    #[test]
    fn delta_test(){
      raise_resource_limits();
      delta_test_me();
    }
    #[test]
    #[serial]
    fn binary_interface_initialization(){
      raise_resource_limits();
      let mut bin_i_1 = new_binary_interface(0,"/home/zack/.local/share/rr/date_viewer-64".to_owned());
      dbg!(bin_i_1.current_frame_time());
      bin_i_1.pin_mut().initialize();
      dbg!(bin_i_1.current_frame_time());
      // let mut bin_i_10 = new_binary_interface(10,"/home/zack/.local/share/rr/date_viewer-64".to_owned());
      // dbg!(bin_i_10.current_frame_time());
      // bin_i_10.pin_mut().initialize();
      // dbg!(bin_i_10.current_frame_time());
      // let mut bin_i_100 = new_binary_interface(100,"/home/zack/.local/share/rr/date_viewer-64".to_owned());
      // dbg!(bin_i_100.current_frame_time());
      // bin_i_100.pin_mut().initialize();
      // dbg!(bin_i_100.current_frame_time());
      // assert_eq!()
    }
}
