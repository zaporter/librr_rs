use std::pin::Pin;
use cxx::private::VectorElement;
use cxx::{type_id, ExternType, let_cxx_string,CxxString, CxxVector, UniquePtr};
use std::path::PathBuf;
use object::{Object, ObjectSection, Segment, Section, SectionKind, ObjectSymbolTable, ObjectSymbol};

unsafe impl ExternType for crate::bindgen::gdbconnection::rr_GdbThreadId {
    type Id = type_id!("rr::GdbThreadId");
    type Kind = cxx::kind::Trivial;
}
unsafe impl ExternType for crate::bindgen::gdbconnection::rr_GdbRegisterValue {
    type Id = type_id!("rr::GdbRegisterValue");
    type Kind = cxx::kind::Trivial;
}
unsafe impl ExternType for crate::bindgen::gdbconnection::rr_GdbRegister {
    type Id = type_id!("rr::GdbRegister");
    type Kind = cxx::kind::Trivial;
}
unsafe impl ExternType for crate::bindgen::gdbconnection::rr_GdbContAction {
    type Id = type_id!("rr::GdbContAction");
    type Kind = cxx::kind::Trivial;
}
unsafe impl ExternType for crate::bindgen::gdbconnection::rr_GdbActionType {
    type Id = type_id!("rr::GdbActionType");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge(namespace="rr")]
mod ffi {

    unsafe extern "C++" {
        include!("librr-rs/src/binary_interface.hpp");
        type GdbThreadId = crate::bindgen::gdbconnection::rr_GdbThreadId;
        type GdbRegisterValue = crate::bindgen::gdbconnection::rr_GdbRegisterValue;
        type GdbRegister = crate::bindgen::gdbconnection::rr_GdbRegister;
        type GdbContAction = crate::bindgen::gdbconnection::rr_GdbContAction;
        type GdbActionType = crate::bindgen::gdbconnection::rr_GdbActionType;
    }

    impl Vec<GdbThreadId>{}
    impl Vec<GdbRegisterValue>{}
    impl CxxVector<GdbRegisterValue>{}

    unsafe extern "C++" {

        #[rust_name="InterfaceRef"]
        type BinaryInterface;
        include!("librr-rs/src/binary_interface.hpp");
        pub fn new_binary_interface(goto_event: i64, trace_dir: String) -> UniquePtr<InterfaceRef>; 
        pub fn initialize(self: Pin<&mut InterfaceRef>) -> bool;
        pub fn current_frame_time(&self) -> i64;
        pub fn get_current_thread(self: &InterfaceRef) -> GdbThreadId;
        #[rust_name="get_auxv_internal"]
        fn get_auxv(self: &InterfaceRef, thread:GdbThreadId) -> &CxxVector<u8>;
        #[rust_name="get_thread_extra_info_internal"]
        fn get_thread_extra_info(&self, thread: GdbThreadId) -> &CxxString;
        #[rust_name="get_register_internal"]
        fn get_register(&self, reg_name: GdbRegister, thread:GdbThreadId) -> &GdbRegisterValue;
        #[rust_name="get_regs_internal"]
        fn get_regs(&self) -> &CxxVector<GdbRegisterValue>;
        pub fn set_sw_breakpoint(self:Pin<&mut InterfaceRef>, addr:usize, kind:i32)->bool;
        pub fn get_thread_list_from_rust(interface: &InterfaceRef) -> Vec<GdbThreadId>;
        pub fn continue_forward(self:Pin<&mut InterfaceRef>, action:GdbContAction) ->bool;
        pub fn setfs_pid(self:Pin<&mut InterfaceRef>, pid:i64);
        #[rust_name="file_read_internal"]
        fn file_read<'a>(self:Pin<&'a mut InterfaceRef>, file_name: &CxxString, flags:i32,mode:i32) ->&'a CxxVector<u8>;
    }
}


pub use ffi::*;
macro_rules! translated_vec {
    ($func_name:ident, $func_name_internal:ident, $extract:ident<$internal:ty> $(,$param:ident: $param_type:ty)*) => {
        pub fn $func_name (self: &InterfaceRef $(,$param: $param_type)*) -> Vec<$internal> {
            $extract::<$internal>(self.$func_name_internal($($param),*))
        }
    };
}
macro_rules! translated_vec_pin_mut {
    ($func_name:ident, $func_name_internal:ident, $extract:ident<$internal:ty> $(,$param:ident: $param_type:ty)*) => {
        pub fn $func_name (self: Pin<&mut InterfaceRef> $(,$param: $param_type)*) -> Vec<$internal> {
            $extract::<$internal>(self.$func_name_internal($($param),*))
        }
    };
}
macro_rules! extracted_fn {
    ($func_name:ident, $func_name_internal:ident, $extract:ident<$internal:ty> -> $out:ty $(,$param:ident: $param_type:ty)*) => {
        pub fn $func_name (self: &InterfaceRef $(,$param: $param_type)*) -> $out {
            $extract::<$internal>(self.$func_name_internal($($param),*))
        }
    };
}
impl InterfaceRef {
    pub fn read_file(self: Pin<&mut InterfaceRef>, file_name:String,flags:i32, mode:i32) -> Vec<u8> {
        let_cxx_string!(file_name_cxx = file_name);
        extract_vec::<u8>(self.file_read_internal(&file_name_cxx, flags,mode))
    }
    // translated_vec_pin_mut!(file_read, file_read_internal, extract_vec<u8>, file_name:&CxxString, flags:i32, mode:i32);
    translated_vec!(get_auxv, get_auxv_internal, extract_vec<u8>, thread:GdbThreadId);
    translated_vec!(get_regs, get_regs_internal, extract_vec<GdbRegisterValue>);
    extracted_fn!(get_thread_extra_info, 
        get_thread_extra_info_internal, 
        extract_str<()> -> String, 
        thread:GdbThreadId);
    extracted_fn!(get_register, 
        get_register_internal,
        extract_clone<GdbRegisterValue> -> GdbRegisterValue,
        reg:GdbRegister,
        thread:GdbThreadId);
}
fn extract_vec<T>(vec: &CxxVector<T>) -> Vec<T> where T : VectorElement +Clone{
    vec.iter().cloned().collect()
}
fn extract_str<T>(strref : &CxxString) -> String {
    strref.to_string()
}
fn extract_clone<T>(object : &T) -> T where T: Clone{
    object.clone()
}
impl InterfaceRef {
    pub fn get_thread_list(&self)->Vec<GdbThreadId>{
        get_thread_list_from_rust(self)
    }
}

custom_derive! {
    #[derive(NewtypeFrom, NewtypeDeref,NewtypeDerefMut)]
    pub struct BinaryInterface(UniquePtr<InterfaceRef>);
}

impl BinaryInterface {
    pub fn new(recording_location: PathBuf) -> Self{
        Self::new_at_target_event(0,recording_location)
    }
    pub fn new_at_target_event(target:i64, recording_location: PathBuf) -> Self{
      let mut bin_interface = ffi::new_binary_interface(target,recording_location.into_os_string().into_string().unwrap());
      bin_interface.pin_mut().initialize();
      Self(bin_interface)
    }

}


#[cfg(test)]
mod tests {
    use crate::raise_resource_limits;
    use crate::record_path_output;
    use serial_test::serial;
    use std::str::FromStr;
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
          let bin_interface = BinaryInterface::new(sample_dateviewer_dir);
        let list = bin_interface.get_thread_list();
        // This sample program has exactly 1 thread
        assert!(list.len()==1);
        // TODO assert that this contains the current thread
        // TODO create a test that has lots of threads and then ensure that it has N threads in the
        // thread list
        dbg!(list);
    }
    #[test]
    #[serial]
    fn file_read_test(){
        initialize();
      let sample_dateviewer_dir = PathBuf::from_str("/home/zack/.local/share/rr/date_viewer-94/").unwrap();
      let mut bin_interface = BinaryInterface::new_at_target_event(0,sample_dateviewer_dir);
      // bin_interface.pin_mut().setfs_pid(688108);
      // let out = bin_interface.pin_mut().read_file("/home/zack/.local/share/rr/date_viewer-94/mmap_hardlink_4_date_viewer".to_owned(),0,0);
      bin_interface.pin_mut().setfs_pid(0);
      let out = bin_interface.pin_mut().read_file("/proc/688108/task/688108/maps".to_owned(),0,448);
     let obj_file = object::File::parse(&*out).unwrap();
     for symbol in obj_file.symbol_table().unwrap().symbols() {
        println!("Name: {}", symbol.name().unwrap());
     }
      dbg!(out);
    }
    
    #[test]
    #[serial]
    fn get_register_list_test(){
        initialize();
          let sample_dateviewer_dir = create_sample_dateviewer_recording();
          let bin_interface = BinaryInterface::new(sample_dateviewer_dir);

        let list = bin_interface.get_regs();
        assert!(list.len() > 10); // we are getting registers
        let eax = bin_interface.get_register(GdbRegister::DREG_EAX,bin_interface.get_current_thread());
        let eip = bin_interface.get_register(GdbRegister::DREG_EIP,bin_interface.get_current_thread());
        let es = bin_interface.get_register(GdbRegister::DREG_ES,bin_interface.get_current_thread());
        for reg in list {
            match reg.name {
                GdbRegister::DREG_EAX => {assert!(reg.get_value_u128()==eax.get_value_u128())},
                GdbRegister::DREG_EIP => {assert!(reg.get_value_u128()==eip.get_value_u128())},
                GdbRegister::DREG_ES => {assert!(reg.get_value_u128()==es.get_value_u128())},
                _=> {}
            };
            dbg!(reg);
        }
        dbg!(eax);
    }
    #[test]
    #[serial]
    fn singlestep_forward_simple_test(){
        initialize();
          let sample_dateviewer_dir = create_sample_dateviewer_recording();

          let mut bin_interface = BinaryInterface::new_at_target_event(150,sample_dateviewer_dir);

        let eip = bin_interface.get_register(GdbRegister::DREG_EIP,bin_interface.get_current_thread());
        let action = GdbContAction{
            type_ : GdbActionType::ACTION_CONTINUE,
            target : bin_interface.get_current_thread(),
            signal_to_deliver : 2,
        };
        dbg!(eip);
        for _ in 0..10 {
        bin_interface.pin_mut().continue_forward(action.clone());
        }
        let eip = bin_interface.get_register(GdbRegister::DREG_EIP,bin_interface.get_current_thread());
        dbg!(eip);
        
        //bin_interface.pin_mut().continue_forward()
    }
    #[test]
    #[serial]
    fn software_breakpoint_test(){
        initialize();
          let sample_dateviewer_dir = create_sample_dateviewer_recording();
          let mut bin_interface = BinaryInterface::new(sample_dateviewer_dir);
          // on x86, this should always be set to 1.
          // dbg!(bin_interface.pin_mut().set_sw_breakpoint(1<<50,1));
          dbg!(bin_interface.get_register(GdbRegister::DREG_EIP,bin_interface.get_current_thread()));
          assert!(bin_interface.pin_mut().set_sw_breakpoint(93941903268112,1));
          // continue
          // EDX = 63
          // EBX = 0 
          // EAX = [33,65,0,28]

    }
    #[test]
    #[serial]
    fn many_starts(){
        initialize();
          let sample_dateviewer_dir = create_sample_dateviewer_recording();
          let mut eip_vals = Vec::new();
          for x in 0..65 {
              let bin_interface = BinaryInterface::new_at_target_event(x*10,sample_dateviewer_dir.clone());
              let eip = bin_interface.get_register(GdbRegister::DREG_EIP,bin_interface.get_current_thread());
              eip_vals.push((x,eip.get_value_u128()));
          }
          dbg!(eip_vals);
          // on x86, this should always be set to 1.
          // dbg!(bin_interface.pin_mut().set_sw_breakpoint(1<<50,1));
          // assert!(bin_interface.pin_mut().set_sw_breakpoint(1<<50,1));
          // continue
          // EDX = 63
          // EBX = 0 
          // EAX = [33,65,0,28]

    }

    #[test]
    #[serial]
    fn binary_interface_creation(){
      initialize();
      let sample_dateviewer_dir = create_sample_dateviewer_recording();
      let mut _bin_interface = BinaryInterface::new(sample_dateviewer_dir);
    }
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
      let sample_dateviewer_dir = create_sample_dateviewer_recording();
      let bin_interface = BinaryInterface::new_at_target_event(500,sample_dateviewer_dir);
      let thread = bin_interface.get_current_thread();
      assert!(thread.pid > 0);
      assert!(thread.tid == thread.pid);
      dbg!(bin_interface.get_current_thread());
    }
    #[test]
    #[serial]
    fn binary_interface_get_extra_thread_info(){
      initialize();
      let sample_dateviewer_dir = create_sample_dateviewer_recording();
      let bin_interface = BinaryInterface::new(sample_dateviewer_dir);
      let thread = bin_interface.get_current_thread();
      dbg!(bin_interface.get_thread_extra_info(thread));
    }
    #[test]
    #[serial]
    fn binary_interface_get_auxv(){
      initialize();
      let sample_dateviewer_dir = create_sample_dateviewer_recording();
      let bin_interface = BinaryInterface::new_at_target_event(500,sample_dateviewer_dir);
      let thread = bin_interface.get_current_thread();
      let auxv = bin_interface.get_auxv(thread);
      // TODO: Put a real test here.
      assert!(auxv.contains(&0));
      assert!(auxv.len()>50);
      assert!(auxv.len()<1000);
      dbg!(auxv);
    }
}
