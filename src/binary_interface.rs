use cxx::private::VectorElement;
use cxx::{let_cxx_string, type_id, CxxString, CxxVector, ExternType, UniquePtr};
use object::{
    Object, ObjectSection, ObjectSymbol, ObjectSymbolTable, Section, SectionKind, Segment,
};
use std::error::Error;
use std::path::PathBuf;
use std::pin::Pin;
use symbolic_common::{Language, Name};
use symbolic_demangle::{Demangle, DemangleOptions};

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

#[cxx::bridge(namespace = "rr")]
mod ffi {

    unsafe extern "C++" {
        include!("librr_rs/src/binary_interface.hpp");
        type GdbThreadId = crate::bindgen::gdbconnection::rr_GdbThreadId;
        type GdbRegisterValue = crate::bindgen::gdbconnection::rr_GdbRegisterValue;
        type GdbRegister = crate::bindgen::gdbconnection::rr_GdbRegister;
        type GdbContAction = crate::bindgen::gdbconnection::rr_GdbContAction;
        type GdbActionType = crate::bindgen::gdbconnection::rr_GdbActionType;
    }

    impl Vec<GdbThreadId> {}
    impl Vec<GdbRegisterValue> {}
    impl CxxVector<GdbRegisterValue> {}

    unsafe extern "C++" {

        #[rust_name = "InterfaceRef"]
        type BinaryInterface;
        include!("librr_rs/src/binary_interface.hpp");
        pub fn new_binary_interface(goto_event: i64, trace_dir: String) -> UniquePtr<InterfaceRef>;
        pub fn initialize(self: Pin<&mut InterfaceRef>) -> bool;
        pub fn current_frame_time(&self) -> i64;
        pub fn get_current_thread(self: &InterfaceRef) -> GdbThreadId;
        #[rust_name = "get_auxv_internal"]
        fn get_auxv(self: &InterfaceRef, thread: GdbThreadId) -> &CxxVector<u8>;
        #[rust_name = "get_thread_extra_info_internal"]
        fn get_thread_extra_info(&self, thread: GdbThreadId) -> &CxxString;
        #[rust_name = "get_register_internal"]
        fn get_register(&self, reg_name: GdbRegister, thread: GdbThreadId) -> &GdbRegisterValue;
        /// Do not use
        #[rust_name = "get_regs_internal"]
        fn get_regs(&self) -> &CxxVector<GdbRegisterValue>;
        pub fn set_sw_breakpoint(self: Pin<&mut InterfaceRef>, addr: usize, kind: i32) -> bool;
        pub fn remove_sw_breakpoint(self: Pin<&mut InterfaceRef>, addr: usize, kind: i32) -> bool;
        pub fn get_thread_list_from_rust(interface: &InterfaceRef) -> Vec<GdbThreadId>;
        /// Continue forward
        pub fn continue_forward(self: Pin<&mut InterfaceRef>, action: GdbContAction) -> bool;
        pub fn continue_backward(self: Pin<&mut InterfaceRef>, action: GdbContAction) -> bool;
        pub fn set_continue_thread(self: Pin<&mut InterfaceRef>, tid: GdbThreadId) -> bool;
        pub fn set_query_thread(self: Pin<&mut InterfaceRef>, tid: GdbThreadId) -> bool;
        pub fn setfs_pid(self: Pin<&mut InterfaceRef>, pid: i64);
        #[rust_name = "set_symbol_internal"]
        fn set_symbol(self: Pin<&mut InterfaceRef>, name: &CxxString, address: usize) -> bool;
        #[rust_name = "get_exec_file_internal"]
        fn get_exec_file(&self) -> &CxxString;

        #[rust_name = "file_read_internal"]
        fn file_read<'a>(
            self: Pin<&'a mut InterfaceRef>,
            file_name: &CxxString,
            flags: i32,
            mode: i32,
        ) -> &'a CxxVector<u8>;
        pub fn add_pass_signal(self: Pin<&mut InterfaceRef>, signal: i32);
        pub fn clear_pass_signals(self: Pin<&mut InterfaceRef>);
        pub fn has_breakpoint_at_address(&self, tuid: GdbThreadId, addr: usize) -> bool;
    }
}

pub use ffi::*;
macro_rules! extract_fn_vec {
    (

        $(#[$meta:meta])*
        pub fn $func_name:ident ($($param:ident: $param_type:ty),*) <- $func_name_internal:ident {
            $extract:ident<$internal:ty>
        } ) => {
        $(#[$meta])*
        pub fn $func_name (self: &InterfaceRef $(,$param: $param_type)*) -> Vec<$internal> {
            $extract::<$internal>(self.$func_name_internal($($param),*))
        }
    };
}
macro_rules! extract_fn {
    (
        $(#[$meta:meta])*
        pub fn $func_name:ident ($($param:ident: $param_type:ty),*) <- $func_name_internal:ident -> $out:ty {
            $extract:ident<$internal:ty>
        } ) => {
        $(#[$meta])*
        pub fn $func_name (self: &InterfaceRef $(,$param: $param_type)*) -> $out {
            $extract::<$internal>(self.$func_name_internal($($param),*))
        }
    };
}
impl InterfaceRef {
    pub fn read_file(
        self: Pin<&mut InterfaceRef>,
        file_name: String,
        flags: i32,
        mode: i32,
    ) -> Vec<u8> {
        let_cxx_string!(file_name_cxx = file_name);
        extract_vec::<u8>(self.file_read_internal(&file_name_cxx, flags, mode))
    }
    pub fn set_symbol(self: Pin<&mut InterfaceRef>, name: String, address: usize) -> bool {
        let_cxx_string!(name_cxx = name);
        self.set_symbol_internal(&name_cxx, address)
    }
    extract_fn_vec!(
        /// Get the auxilary vector 
        pub fn get_auxv (thread:GdbThreadId) <- get_auxv_internal {
            extract_vec<u8>
        }   
    );
    extract_fn_vec!(
        /// Get a vector of all of the registers. 
        /// This may include strangely named or unused registers
        pub fn get_regs () <- get_regs_internal {
            extract_vec<GdbRegisterValue>
        }   
    );

    extract_fn!(
        pub fn get_thread_extra_info(thread:GdbThreadId) <- get_thread_extra_info_internal -> String {
            extract_str<()>
        }
    );
    extract_fn!(
        /// Return a file path that contains all of the symbols for the executable
        /// This is often a clone of the orignal executable because the original
        /// might have moved. 
        pub fn get_exec_file() <- get_exec_file_internal -> String {
            extract_str<()>
        }
    );
    extract_fn!(
        /// Get value of a register
        pub fn get_register(reg:GdbRegister, thread:GdbThreadId) <- get_register_internal -> GdbRegisterValue {
            extract_clone<GdbRegisterValue>
        }
    );
    pub fn get_thread_list(&self) -> Vec<GdbThreadId> {
        get_thread_list_from_rust(self)
    }

}
fn extract_vec<T>(vec: &CxxVector<T>) -> Vec<T>
where
    T: VectorElement + Clone,
{
    vec.iter().cloned().collect()
}
fn extract_str<T>(strref: &CxxString) -> String {
    strref.to_string()
}
fn extract_clone<T>(object: &T) -> T
where
    T: Clone,
{
    object.clone()
}

custom_derive! {
    #[derive(NewtypeFrom, NewtypeDeref,NewtypeDerefMut)]
    pub struct BinaryInterface(UniquePtr<InterfaceRef>);
}

impl BinaryInterface {
    pub fn new(recording_location: PathBuf) -> Self {
        Self::new_at_target_event(0, recording_location)
    }
    pub fn new_at_target_event(target: i64, recording_location: PathBuf) -> Self {
        let mut bin_interface = ffi::new_binary_interface(
            target,
            recording_location.into_os_string().into_string().unwrap(),
        );

        bin_interface.pin_mut().initialize();
        Self(bin_interface)
    }
    pub fn get_proc_map(&mut self) -> Result<procmaps::Mappings, Box<dyn Error>> {
        // Load Proc Map
        self.pin_mut().setfs_pid(0);
        let current_thread = self.get_current_thread();
        let out = self.pin_mut().read_file(
            format!(
                "/proc/{}/task/{}/maps",
                current_thread.pid, current_thread.tid
            ),
            0,
            448,
        );
        let data = std::str::from_utf8(&out)?;
        Ok(procmaps::Mappings::from_str(data)?)
        //
    }
    pub fn set_pass_signals(&mut self, signals: Vec<i32>) {
        self.pin_mut().clear_pass_signals();
        for signal in signals {
            self.pin_mut().add_pass_signal(signal);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::raise_resource_limits;
    use crate::record_path_output;
    use gag::BufferRedirect;
    use rand::prelude::*;
    use serial_test::serial;
    use std::io::Read;
    use std::str::FromStr;
    use std::{path::PathBuf, sync::Once};
    static INIT: Once = Once::new();

    fn initialize() {
        INIT.call_once(|| {
            raise_resource_limits();
        });
    }
    fn create_sample_dateviewer_recording() -> PathBuf {
        let exe_dir = std::env::current_dir()
            .unwrap()
            .join("test-executables/build")
            .join("date_viewer");
        let random_number: u64 = rand::thread_rng().gen();
        let save_dir = std::env::temp_dir().join(random_number.to_string());
        let mut output = String::new();
        let mut stdout_buf = BufferRedirect::stdout().unwrap();
        let ret_code = record_path_output(
            exe_dir.into_os_string().into_string().unwrap(),
            Some(vec![100_u32.to_string()]),
            save_dir.clone().into_os_string().into_string().unwrap(),
        );
        stdout_buf.read_to_string(&mut output).unwrap();
        drop(stdout_buf);
        assert!(output.contains("Started"));
        assert!(output.contains("StartTime"));
        assert!(output.contains("EndTime"));
        assert!(output.contains("Finished"));
        assert_eq!(ret_code, 0);
        save_dir
    }

    #[test]
    #[serial]
    fn thread_list_test() {
        initialize();
        let sample_dateviewer_dir = create_sample_dateviewer_recording();
        let bin_interface = BinaryInterface::new(sample_dateviewer_dir);
        let list = bin_interface.get_thread_list();
        // This sample program has exactly 1 thread
        assert!(list.len() == 1);
        // TODO assert that this contains the current thread
        // TODO create a test that has lots of threads and then ensure that it has N threads in the
        // thread list
        dbg!(list);
    }
    #[test]
    #[serial]
    fn file_read_test() {
        initialize();
        let sample_dateviewer_dir = create_sample_dateviewer_recording();
        // let sample_dateviewer_dir =
        //     PathBuf::from_str("/home/zack/.local/share/rr/date_viewer-94").unwrap();
        let mut bin_interface = BinaryInterface::new_at_target_event(0, sample_dateviewer_dir);

        let mut found_mapping = false;
        let current_thread = bin_interface.get_current_thread();
        let mappings = bin_interface.get_proc_map().unwrap();
        let rip = bin_interface
            .get_register(GdbRegister::DREG_RIP, current_thread.clone())
            .get_value_u128() as usize;
        for mapping in mappings.iter() {
            if rip < mapping.ceiling && rip > mapping.base {
                assert!(mapping.perms.readable);
                assert!(!mapping.perms.writable);
                assert!(mapping.perms.executable);
                // should be ld-linux-x86-64.so.2
                dbg!(rip);
                dbg!(mapping);
                found_mapping = true;
            }
            dbg!(mapping);
        }

        assert!(found_mapping);
        let symbol_file =
            bin_interface.get_exec_file();
        dbg!(&symbol_file);

            //"/home/zack/.local/share/rr/DateTester-44/mmap_hardlink_4_DateTester".to_owned();
        let symbol_str = std::fs::read(symbol_file).unwrap();
        let obj_file = object::File::parse(&*symbol_str).unwrap();
        let mut possible_mains = Vec::new();
        for symbol in obj_file.symbol_table().unwrap().symbols() {
            // println!("Name: {}", symbol.name().unwrap());
            if symbol.name().unwrap().contains("main") {
                let name : String = Name::from(symbol.name().unwrap()).try_demangle(DemangleOptions::name_only()).to_string();
                possible_mains.push((name, symbol.address() as usize));
            }
        }
        // for (name,addr) in possible_mains {
        //     dbg!(&name);


        // }
        let main_addr = possible_mains.into_iter().filter(|k| k.0 == "date_viewer::main").next().unwrap().1;
        dbg!(main_addr);
        let addr_1 = 94397935460352 as usize;
        let addr_2 = 94397935484928 as usize;
        let addr_3 = 94397935722496 as usize;
        let addr_4 = 94397935783936 as usize;

        dbg!(bin_interface.pin_mut().set_sw_breakpoint(addr_1+main_addr, 1));
        dbg!(bin_interface.pin_mut().set_sw_breakpoint(addr_2+main_addr, 1));
        dbg!(bin_interface.pin_mut().set_sw_breakpoint(addr_3+main_addr, 1));
        dbg!(bin_interface.pin_mut().set_sw_breakpoint(addr_4+main_addr, 1));
        // assert!(bin_interface.pin_mut().set_sw_breakpoint(0x55dc6aaf28f0, 1));
        // assert!(bin_interface.pin_mut().set_sw_breakpoint(0x7f2ae7ef1300, 1));
        bin_interface.set_pass_signals(vec![
            0xe, 0x14, 0x17, 0x1a, 0x1b, 0x1c, 0x21, 0x24, 0x25, 0x2c, 0x4c, 0x97,
        ]);

        let eip =
            bin_interface.get_register(GdbRegister::DREG_RIP, bin_interface.get_current_thread());
        dbg!(bin_interface.get_register(GdbRegister::DREG_EIP, bin_interface.get_current_thread()));
        let action = GdbContAction {
            type_: GdbActionType::ACTION_STEP,
            target: bin_interface.get_current_thread(),
            signal_to_deliver: 0,
        };
        let action2 = GdbContAction {
            type_: GdbActionType::ACTION_CONTINUE,
            target: bin_interface.get_current_thread(),
            signal_to_deliver: 0,
        };
        dbg!(eip);
        for _ in 0..10 {
            bin_interface.pin_mut().continue_forward(action.clone());
            bin_interface.pin_mut().continue_forward(action2.clone());
            let eip = bin_interface
                .get_register(GdbRegister::DREG_RIP, bin_interface.get_current_thread());
            dbg!(eip);
        }
        dbg!(bin_interface.get_exec_file());
    }

    #[test]
    #[serial]
    fn get_register_list_test() {
        initialize();
        let sample_dateviewer_dir = create_sample_dateviewer_recording();
        let bin_interface = BinaryInterface::new(sample_dateviewer_dir);

        let list = bin_interface.get_regs();
        assert!(list.len() > 10); // we are getting registers
        let eax =
            bin_interface.get_register(GdbRegister::DREG_EAX, bin_interface.get_current_thread());
        let eip =
            bin_interface.get_register(GdbRegister::DREG_RIP, bin_interface.get_current_thread());
        let es =
            bin_interface.get_register(GdbRegister::DREG_ES, bin_interface.get_current_thread());
        for reg in list {
            match reg.name {
                GdbRegister::DREG_EAX => {
                    assert!(reg.get_value_u128() == eax.get_value_u128())
                }
                GdbRegister::DREG_RIP => {
                    assert!(reg.get_value_u128() == eip.get_value_u128())
                }
                GdbRegister::DREG_ES => {
                    assert!(reg.get_value_u128() == es.get_value_u128())
                }
                _ => {}
            };
            dbg!(reg);
        }
        dbg!(eax);
    }
    #[test]
    #[ignore]
    #[serial]
    fn singlestep_forward_simple_test() {
        initialize();
        let sample_dateviewer_dir = create_sample_dateviewer_recording();

        let mut bin_interface = BinaryInterface::new_at_target_event(150, sample_dateviewer_dir);

        let eip =
            bin_interface.get_register(GdbRegister::DREG_RIP, bin_interface.get_current_thread());
        let action = GdbContAction {
            type_: GdbActionType::ACTION_STEP,
            target: bin_interface.get_current_thread(),
            signal_to_deliver: 0,
        };
        dbg!(eip);
        for _ in 0..10 {
            bin_interface.pin_mut().continue_forward(action.clone());
        }
        let eip =
            bin_interface.get_register(GdbRegister::DREG_RIP, bin_interface.get_current_thread());
        dbg!(eip);

        //bin_interface.pin_mut().continue_forward()
    }
    #[test]
    #[serial]
    fn software_breakpoint_test() {
        initialize();
        let sample_dateviewer_dir = create_sample_dateviewer_recording();
        let mut bin_interface = BinaryInterface::new(sample_dateviewer_dir);
        // on x86, this should always be set to 1.
        // dbg!(bin_interface.pin_mut().set_sw_breakpoint(1<<50,1));
        dbg!(bin_interface.get_register(GdbRegister::DREG_EIP, bin_interface.get_current_thread()));
        dbg!(bin_interface.get_exec_file());
        // assert!(bin_interface.pin_mut().set_sw_breakpoint(93941903268112, 1));
        // continue
        // EDX = 63
        // EBX = 0
        // EAX = [33,65,0,28]
    }
    #[test]
    #[serial]
    fn many_starts() {
        initialize();
        let sample_dateviewer_dir = create_sample_dateviewer_recording();
        let mut eip_vals = Vec::new();
        for x in 0..65 {
            let bin_interface =
                BinaryInterface::new_at_target_event(x * 10, sample_dateviewer_dir.clone());
            let eip = bin_interface
                .get_register(GdbRegister::DREG_EIP, bin_interface.get_current_thread());
            eip_vals.push((x, eip.get_value_u128()));
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
    fn binary_interface_creation() {
        initialize();
        let sample_dateviewer_dir = create_sample_dateviewer_recording();
        let mut _bin_interface = BinaryInterface::new(sample_dateviewer_dir);
    }
    #[test]
    #[serial]
    fn binary_interface_initialization_dateviewer_0() {
        initialize();
        let sample_dateviewer_dir = create_sample_dateviewer_recording();
        let mut bin_interface = new_binary_interface(
            0,
            sample_dateviewer_dir
                .into_os_string()
                .into_string()
                .unwrap(),
        );
        assert_eq!(bin_interface.current_frame_time(), 1);
        let mut output = String::new();
        let mut stdout_buf = BufferRedirect::stdout().unwrap();
        bin_interface.pin_mut().initialize();
        stdout_buf.read_to_string(&mut output).unwrap();
        drop(stdout_buf);
        assert!(!output.contains("Started"));
        assert!(bin_interface.current_frame_time() >= 1);
    }
    #[test]
    #[serial]
    fn binary_interface_initialization_dateviewer_1000() {
        initialize();
        let sample_dateviewer_dir = create_sample_dateviewer_recording()
            .into_os_string()
            .into_string()
            .unwrap();
        let mut bin_interface = new_binary_interface(10000, sample_dateviewer_dir);
        assert_eq!(bin_interface.current_frame_time(), 1);
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
    fn binary_interface_get_current_thread() {
        initialize();
        let sample_dateviewer_dir = create_sample_dateviewer_recording();
        let bin_interface = BinaryInterface::new_at_target_event(500, sample_dateviewer_dir);
        let thread = bin_interface.get_current_thread();
        assert!(thread.pid > 0);
        assert!(thread.tid == thread.pid);
        dbg!(bin_interface.get_current_thread());
    }
    #[test]
    #[serial]
    fn binary_interface_get_extra_thread_info() {
        initialize();
        let sample_dateviewer_dir = create_sample_dateviewer_recording();
        let bin_interface = BinaryInterface::new(sample_dateviewer_dir);
        let thread = bin_interface.get_current_thread();
        dbg!(bin_interface.get_thread_extra_info(thread));
    }
    #[test]
    #[serial]
    fn binary_interface_get_auxv() {
        initialize();
        let sample_dateviewer_dir = create_sample_dateviewer_recording();
        let bin_interface = BinaryInterface::new_at_target_event(500, sample_dateviewer_dir);
        let thread = bin_interface.get_current_thread();
        let auxv = bin_interface.get_auxv(thread);
        // TODO: Put a real test here.
        assert!(auxv.contains(&0));
        assert!(auxv.len() > 50);
        assert!(auxv.len() < 1000);
        dbg!(auxv);
    }
}
