use std::u128;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Debug;
use cxx::UniquePtr;

use cxx::{type_id, ExternType, CxxString, CxxVector};
use autocxx::prelude::*;

// autocxx::include_cpp! {
//     // #include "src/binary_interface.hpp"
//     generate!("rr::ReplayResult")
//     safety!(unsafe_ffi)
// }

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
        pub fn initialize(self: Pin<&mut BinaryInterface>) -> bool;
        pub fn current_frame_time(&self) -> i64;
        // pub fn set_software_breakpoint(self : Pin<&mut BinaryInterface>, tid:Option<GdbThreadId>) -> i32;
        // pub fn get_thread_list(self: &BinaryInterface) -> Vec<GdbThreadId>;
        pub fn get_current_thread(self: &BinaryInterface) -> GdbThreadId;
        pub fn get_auxv(self: &BinaryInterface, thread:GdbThreadId) -> &CxxVector<u8>;
        // pub fn get_regs(self: &BinaryInterface, tid: i32) -> Vec<GdbRegisterValue>;
        // pub fn get_exec_file(self: &BinaryInterface, request_target : GdbThreadId) -> String;
        pub fn rstrans_get_regs(interface: &BinaryInterface) -> Vec<GdbRegisterValue>;
        pub fn get_thread_list_from_rust(interface: &BinaryInterface) -> Vec<GdbThreadId>;
    }
}
fn extract_vec(vec: &CxxVector<u8>) -> Vec<u8> {
    vec.iter().map(|val| val.to_owned()).collect()
}
impl BinaryInterface {
    pub fn get_thread_list(&self)->Vec<GdbThreadId>{
        get_thread_list_from_rust(self)
    }
    pub fn get_regs(&self)->Vec<GdbRegisterValue>{
        rstrans_get_regs(self)
    }
}
pub use binary_interface_ffi::*;



#[cfg(test)]
mod tests {
    use crate::raise_resource_limits;
    use crate::record_path_output;
    use serial_test::serial;
    use std::{sync::Once, path::PathBuf};
    use rand::prelude::*;
    use gag::BufferRedirect;
    use std::io::Read;
    use super::*;
    static INIT: Once = Once::new();

    fn initialize(){
        INIT.call_once(|| {
            raise_resource_limits();
        }); 
    }
    fn create_sample_dateviewer_recording() -> PathBuf {
        let exe_dir = std::env::current_dir().unwrap().join("test-executables/build").join("date_viewer");
        let random_number: u64 = rand::thread_rng().gen();
        let save_dir = std::env::temp_dir().join(random_number.to_string());
        let mut output = String::new();
        let mut stdout_buf = BufferRedirect::stdout().unwrap(); 
        let ret_code = record_path_output(
            exe_dir.into_os_string().into_string().unwrap(),
            Some(vec![100_u32.to_string()]),
            save_dir.clone().into_os_string().into_string().unwrap());
        stdout_buf.read_to_string(&mut output).unwrap();
        drop(stdout_buf);
        assert!(output.contains("Started"));
        assert!(output.contains("StartTime"));
        assert!(output.contains("EndTime"));
        assert!(output.contains("Finished"));
        assert_eq!(ret_code,0);
        save_dir
    }

    
    #[test]
    #[serial]
    fn thread_list_test(){
        initialize();
          let sample_dateviewer_dir = create_sample_dateviewer_recording();
          let mut bin_interface = new_binary_interface(0,sample_dateviewer_dir.into_os_string().into_string().unwrap());
          assert_eq!(bin_interface.current_frame_time(),1);
          bin_interface.pin_mut().initialize();
        println!("Hi!");
        let list = bin_interface.get_thread_list();
        dbg!(list);
    }
    
    #[test]
    #[serial]
    fn register_list_test(){
        initialize();
          let sample_dateviewer_dir = create_sample_dateviewer_recording();
          let mut bin_interface = new_binary_interface(50,sample_dateviewer_dir.into_os_string().into_string().unwrap());
          bin_interface.pin_mut().initialize();

        let list = bin_interface.get_regs();
        assert!(list.len() > 10); // we are getting registers
        // TODO : Improve this test by adding checks for EAX, RIP, etc and ensuring valid values. 
        for reg in list {
            dbg!(reg);
        }
    }

    #[test]
    #[serial]
    fn binary_interface_creation(){
      initialize();
      let sample_dateviewer_dir = create_sample_dateviewer_recording();
      let _bin_interface = new_binary_interface(0,sample_dateviewer_dir.into_os_string().into_string().unwrap());
    }
    // TODO These tests are important. Please bring them back.
    // #[test]
    // fn beta_test(){
    //   raise_resource_limits();
    //   beta_test_me();
    // }
    // #[test]
    // fn gamma_test(){
    //   raise_resource_limits();
    //   gamma_test_me();
    // }
    #[test]
    #[serial]
    fn binary_interface_initialization_dateviewer_0(){
      initialize();
      let sample_dateviewer_dir = create_sample_dateviewer_recording();
      let mut bin_interface = new_binary_interface(0,sample_dateviewer_dir.into_os_string().into_string().unwrap());
      assert_eq!(bin_interface.current_frame_time(),1);
      let mut output = String::new();
      let mut stdout_buf = BufferRedirect::stdout().unwrap(); 
      bin_interface.pin_mut().initialize();
      stdout_buf.read_to_string(&mut output).unwrap();
      drop(stdout_buf);
      assert!(!output.contains("Started"));
      assert!(bin_interface.current_frame_time() >= 1);
    }
    #[test]
    #[ignore]
    #[serial]
    fn binary_interface_initialization_dateviewer_660(){
      initialize();
      let sample_dateviewer_dir = create_sample_dateviewer_recording();
      let mut bin_interface = new_binary_interface(660,sample_dateviewer_dir.into_os_string().into_string().unwrap());
      assert_eq!(bin_interface.current_frame_time(),1);
      let mut output = String::new();
      let mut stdout_buf = BufferRedirect::stdout().unwrap(); 
      bin_interface.pin_mut().initialize();
      stdout_buf.read_to_string(&mut output).unwrap();
      drop(stdout_buf);
      assert!(output.contains("Started"));
      assert!(!output.contains("Finished"));
      assert!(bin_interface.current_frame_time() >= 660);
    }
    #[test]
    #[serial]
    fn binary_interface_initialization_dateviewer_1000(){
      initialize();
      let sample_dateviewer_dir = create_sample_dateviewer_recording().into_os_string().into_string().unwrap();
      let mut bin_interface = new_binary_interface(1000,sample_dateviewer_dir);
      assert_eq!(bin_interface.current_frame_time(),1);
      let mut output = String::new();
      let mut stdout_buf = BufferRedirect::stdout().unwrap(); 
      bin_interface.pin_mut().initialize();
      stdout_buf.read_to_string(&mut output).unwrap();
      drop(stdout_buf);
      assert!(output.contains("Started"));
      assert!(output.contains("Finished"));
      dbg!(bin_interface.current_frame_time());
      assert!(bin_interface.current_frame_time() >= 660);
    }
    #[test]
    #[serial]
    fn binary_interface_get_current_thread(){
      initialize();
      let sample_dateviewer_dir = create_sample_dateviewer_recording().into_os_string().into_string().unwrap();
      let mut bin_interface = new_binary_interface(500,sample_dateviewer_dir);
      bin_interface.pin_mut().initialize();
      let thread = bin_interface.get_current_thread();
      assert!(thread.pid > 0);
      assert!(thread.tid == thread.pid);
      dbg!(bin_interface.get_current_thread());

    }
    #[test]
    #[serial]
    fn binary_interface_get_auxv(){
      initialize();
      let sample_dateviewer_dir = create_sample_dateviewer_recording().into_os_string().into_string().unwrap();
      let mut bin_interface = new_binary_interface(500,sample_dateviewer_dir);
      bin_interface.pin_mut().initialize();
      let thread = bin_interface.get_current_thread();
      let auxv = extract_vec(bin_interface.get_auxv(thread));
      dbg!(auxv);
    }
}
