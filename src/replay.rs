use cxx::UniquePtr;
use std::pin::Pin;

#[cxx::bridge]
pub mod replayffi {


    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct ReplayingFlags {
        goto_event : i64,
        singlestep_to_event : i64,
        target_process : i32,
        // TODO ZACK: process_created_how
        dont_launch_debugger : bool,
        dbg_port : i32,
        dbg_host : String,
        keep_listening : bool,
        gdb_options : Vec<String>,
        gdb_binary_file_path : String,
        redirect : bool,
        cpu_unbound : bool,
        share_private_mappings : bool,
        dump_interval : u32,
        serve_files : bool,
        tty : String,

    }
    unsafe extern "C++" {
        include!("librr-rs/src/replay.hpp");
        pub fn replay(flags : ReplayingFlags, tracedir : String) -> i32;
        pub fn get_default_replay_flags() -> ReplayingFlags;
        fn replay_flags_pipe_test(flags: ReplayingFlags) -> ReplayingFlags;
    }
    #[namespace = "rr" ]
    unsafe extern "C++" {
        include!("librr-rs/src/replay.hpp");
        type ReplayController;
        fn print_test_controller(&self);
        fn test_run(self : Pin<&mut ReplayController>);
        fn can_continue_replay(&self) -> bool;
        fn setup(self : Pin<&mut ReplayController>);
        fn new_replay_controller(trace_dir : String, flags:ReplayingFlags) -> UniquePtr<ReplayController>;
    }
}

pub use replayffi::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn replay_flags_fuck(){
        let flags = get_default_replay_flags();
        let mut controller = new_replay_controller("".to_owned(), flags);
        controller.print_test_controller();
        controller.pin_mut().setup();
        println!("can continue replay: {}", controller.can_continue_replay());
    }
    #[test]
    fn replay_controller_setup(){
        let flags = get_default_replay_flags();
        let mut controller = new_replay_controller("".to_owned(), flags);
        controller.print_test_controller();
        controller.pin_mut().setup();
        assert!(controller.can_continue_replay())
    }
    #[test]
    fn replay_flags_defaults(){
        //test_replay();
        let flags = get_default_replay_flags();
        assert_eq!(flags.dont_launch_debugger, false);
        assert_eq!(flags.goto_event, 0);
        assert_eq!(flags.singlestep_to_event, 0);
        assert_eq!(flags.target_process,0);
        assert_eq!(flags.dbg_port, -1);
        assert_eq!(flags.dbg_host, "127.0.0.1");
        assert_eq!(flags.keep_listening,false);
        assert_eq!(flags.redirect,true);
        assert_eq!(flags.cpu_unbound, false);
        assert_eq!(flags.share_private_mappings, false);
        assert_eq!(flags.dump_interval, 0);
        assert_eq!(flags.serve_files, false);
    }
    #[test]
    fn replay_flags_pipe_test_1(){
        let mut flags = get_default_replay_flags();
        flags.dont_launch_debugger = true;
        flags.goto_event = -1;
        flags.singlestep_to_event = 100;
        flags.target_process = 1;
        flags.dbg_port = 10;
        flags.dbg_host = "The Lands Between".to_owned();
        flags.keep_listening = true;
        flags.gdb_options.push("Try".to_owned());
        flags.gdb_options.push("Attacking".to_owned());
        flags.gdb_binary_file_path = "/bin/elden_ring".to_owned();
        flags.redirect = false;
        flags.cpu_unbound = true;
        flags.share_private_mappings = true;
        flags.dump_interval = 10;
        flags.serve_files = true;
        flags.tty = "Torrent".to_owned(); 
        let response = replay_flags_pipe_test(flags.clone());
        assert_eq!(flags, response);
    }

}
