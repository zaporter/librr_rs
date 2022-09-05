#[cxx::bridge(namespace="rr")]
pub mod recordffi {
    //pub struct DisableCPUIDFeatures {
    //    features_ecx : u32,
    //    features_edx : u32,
    //    extended_features_ebx : u32,
    //    extended_features_ecx : u32,
    //    extended_features_edx : u32,
    //    xsave_features_eax : u32,
    //}
    //enum NestedBehavior {
    //  NESTED_ERROR,
    //  NESTED_IGNORE,
    //  NESTED_DETACH,
    //  NESTED_RELEASE,
    //}*/


    //#[derive(Debug, PartialEq, Eq, Clone)]
    //pub struct RecordingFlags {
    //    extra_env : Vec<String>,
    //    max_ticks : i64,
    //    ignore_sig : i32,
    //    continue_through_sig : i32,
    //    use_syscall_buffer : bool,
    //    syscall_buffer_size : usize,
    //    // TODO disable_cpuid_features : DisableCPUIDFeatures,
    //    print_trace_dir : i32,
    //    output_trace_dir : String,
    //    use_file_cloning : bool,
    //    use_read_cloning : bool,
    //    /**
    //     * BIND_CPU (-2) means binding to a randomly chosen CPU.
    //     * UNBOUND_CPU (-1) means not binding to a particular CPU.
    //     * A non-negative value means binding to the specific CPU number.
    //     */
    //    bind_cpu : i32,
    //    always_switch : bool,
    //    chaos : bool,
    //    num_cores : i32,
    //    wait_for_all : bool,
    //    //TODO nested : NestedBehavior,
    //    nested : i32,
    //    scarce_fds : bool,
    //    setuid_sudo : bool,
    //    //TODO trace_id,
    //    copy_preload_src : bool,
    //    syscallbuf_desched_sig : i32,
    //    stap_sdt : bool,
    //    unmap_vdso : bool,
    //    asan : bool,
        
    //}

    unsafe extern "C++" {
        include!("librr_rs/src/record.hpp");
        // pub fn get_default_record_flags() -> RecordingFlags;
        pub fn record(args : Vec<String>) -> i32;
        // fn record_flags_pipe_test(flags : RecordingFlags) -> RecordingFlags;
    }
}
pub fn record_path_output(executable:String, executable_args: Option<Vec<String>>,output_dir:String)->i32{
    let mut args = vec!["--output-trace-dir".to_owned(), output_dir,executable];
    if let Some(mut exe_args) = executable_args{
        args.append(&mut exe_args);
    }
    record(args)
}
pub use recordffi::*;

#[cfg(test)]
mod tests {
    use crate::raise_resource_limits;
    use serial_test::serial;
    use std::sync::Once;
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

    #[test]
    #[serial]
    fn basic_record_dateviewer(){
        initialize();
        let exe_dir = std::env::current_dir().unwrap().join("test-executables/build").join("date_viewer");
        let random_number: u64 = rand::thread_rng().gen();
        let save_dir = std::env::temp_dir().join(random_number.to_string());
        let mut output = String::new();
        let mut stdout_buf = BufferRedirect::stdout().unwrap(); 
        let ret_code = record_path_output(
            exe_dir.into_os_string().into_string().unwrap(),
            None,
            save_dir.into_os_string().into_string().unwrap());
        stdout_buf.read_to_string(&mut output).unwrap();
        drop(stdout_buf);
        assert!(output.contains("Started"));
        assert!(output.contains("StartTime"));
        assert!(!output.contains("EndTime"));
        assert!(output.contains("Finished"));
        assert_eq!(ret_code,0);
    }

    #[test]
    #[serial]
    fn basic_record_dateviewer_args(){
        initialize();
        let exe_dir = std::env::current_dir().unwrap().join("test-executables/build").join("date_viewer");
        let random_number: u64 = rand::thread_rng().gen();
        let save_dir = std::env::temp_dir().join(random_number.to_string());
        let mut output = String::new();
        let mut stdout_buf = BufferRedirect::stdout().unwrap(); 
        let ret_code = record_path_output(
            exe_dir.into_os_string().into_string().unwrap(),
            Some(vec![100_u32.to_string()]),
            save_dir.into_os_string().into_string().unwrap());
        stdout_buf.read_to_string(&mut output).unwrap();
        drop(stdout_buf);
        assert!(output.contains("Started"));
        assert!(output.contains("StartTime"));
        assert!(output.contains("EndTime"));
        assert!(output.contains("Finished"));
        assert_eq!(ret_code,0);
    }

    // #[test]
    // fn record_flags_defaults(){
    //     let flags = get_default_record_flags();
    //     assert_eq!(flags.extra_env.len(), 0);
    //     assert_eq!(flags.max_ticks, 2500000);
    //     assert_eq!(flags.ignore_sig, 0);
    //     assert_eq!(flags.continue_through_sig, 0);
    //     assert_eq!(flags.use_syscall_buffer,true);
    //     assert_eq!(flags.syscall_buffer_size, 0);
    //     assert_eq!(flags.print_trace_dir, -1);
    //     assert_eq!(flags.output_trace_dir, "");
    //     assert_eq!(flags.use_file_cloning, true);
    //     assert_eq!(flags.use_read_cloning, true);
    //     assert_eq!(flags.bind_cpu, -2);
    //     assert_eq!(flags.always_switch, false);
    //     assert_eq!(flags.chaos, false);
    //     assert_eq!(flags.num_cores, 0);
    //     assert_eq!(flags.wait_for_all, false);
    //     assert_eq!(flags.nested,0);
    //     assert_eq!(flags.scarce_fds, false);
    //     assert_eq!(flags.setuid_sudo,false);
    //     assert_eq!(flags.copy_preload_src, false);
    //     assert_eq!(flags.syscallbuf_desched_sig,30);
    //     assert_eq!(flags.stap_sdt, false);
    //     assert_eq!(flags.unmap_vdso,false);
    //     assert_eq!(flags.asan, false);
    // }
    // #[test]
    // fn record_flags_pipe_test_1(){
    //     let mut flags = get_default_record_flags();
    //     flags.extra_env.push("Hello".to_owned());
    //     flags.extra_env.push("Programmer!".to_owned());
    //     flags.max_ticks = 221;
    //     flags.ignore_sig = 10;
    //     flags.continue_through_sig = -50;
    //     flags.use_syscall_buffer = false;
    //     flags.syscall_buffer_size = 10;
    //     flags.print_trace_dir = 100;
    //     flags.output_trace_dir = "Some/path".to_owned();
    //     flags.use_file_cloning = false;
    //     flags.use_read_cloning = false;
    //     flags.bind_cpu = 7;
    //     flags.always_switch = true;
    //     flags.chaos = true;
    //     flags.num_cores = 12;
    //     flags.wait_for_all = true;
    //     flags.nested = 3;
    //     flags.scarce_fds = true;
    //     flags.setuid_sudo = true;
    //     flags.copy_preload_src = true;
    //     flags.syscallbuf_desched_sig = 1000;
    //     flags.stap_sdt = true;
    //     flags.unmap_vdso = true;
    //     flags.asan = true;
    //     let response = record_flags_pipe_test(flags.clone());
    //     assert_eq!(flags, response);

    // }
}
