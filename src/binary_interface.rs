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
        pub fn new_binary_interface(trace_dir: String) -> UniquePtr<BinaryInterface>; 
        // pub fn initialize(self: Pin<&mut BinaryInterface>) -> bool;
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
    fn get_regs_test(){
      raise_resource_limits();
      // let mut flags = ReplayingFlags::default();
      // flags.goto_event = 1;
      // flags.dont_launch_debugger = true;
      // dbg!(&flags);
      let mut bin_interface = new_binary_interface("/home/zack/.local/share/rr/DateTester-41".to_owned());
      // dbg!(bin_interface.pin_mut().initialize());
      // let threads = bin_interface.get_thread_list();
      // dbg!(threads);
      // let mut current_thread = bin_interface.get_current_thread();
      // current_thread.tid = 48226;
      // dbg!(bin_interface.get_exec_file(current_thread));
      // dbg!(bin_interface.get_regs(0).len());

    }
}
