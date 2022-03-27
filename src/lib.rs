//TODO:
//Use throw catch to return Result<> types instead of c style return codes.

mod zags;
mod record;
mod replay;
mod librr;

#[allow(warnings)]
mod bindgen;


pub use record::*;
pub use replay::*;
pub use librr::*;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn simple_test_main(){
        assert_eq!(5,1+4);
    }
    #[ignore]
    #[test]
    fn full_record_test(){
        raise_resource_limits();
        let flags = get_default_record_flags();
        println!("{:?}", flags);
        let retval= record(vec!["/home/zack/date_viewer".to_owned()], flags);
        //let retval= record(vec!["firefox".to_owned()], flags);
        assert_eq!(retval,0);
    }
    #[ignore]
    #[test]
    fn full_replay_test(){
        raise_resource_limits();
        let mut flags = get_default_replay_flags();
        flags.goto_event = i64::MAX;
        //flags.singlestep_to_event = 1;
        //flags.dump_interval = 400;
        flags.dont_launch_debugger = true;
        println!("{:?}", flags);
        let retval = replay(flags, "".to_owned());
        assert_eq!(retval,0);
    }
    #[ignore]
    #[test]
    fn full_binary_connection_test(){
        raise_resource_limits();
        let mut flags = get_default_replay_flags();
        flags.goto_event = 535;
        //flags.singlestep_to_event = 1;
        //flags.dump_interval = 400;
        flags.dont_launch_debugger = true;
        println!("{:?}", flags);
        let retval = replay(flags, "".to_owned());
        assert_eq!(retval,0);
    }

}
