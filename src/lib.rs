mod zags;
mod record;
mod replay;
mod librr;

use record::*;
use replay::*;
use librr::*;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn simple_test_main(){
        assert_eq!(5,1+4);
    }
    #[test]
    fn full_record_test(){
        raise_resource_limits();
        let flags = get_default_record_flags();
        println!("{:?}", flags);
        let retval= record(vec!["/home/zack/date_viewer".to_owned()], flags);
        assert_eq!(retval,0);
    }

}
