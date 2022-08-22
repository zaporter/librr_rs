pub mod gdbconnection {
    include!(concat!(env!("OUT_DIR"), "/gdbconnection-bindings.rs"));
}
pub mod taskishuid {
    include!(concat!(env!("OUT_DIR"), "/taskishuid-bindings.rs"));
}
// pub mod replaysession {
//     include!(concat!(env!("OUT_DIR"), "/replaysession-bindings.rs"));
// }

use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Debug;
use cxx::UniquePtr;
use crate::bindgen::gdbconnection::rr_GdbRegisterValue;

impl rr_GdbRegisterValue{
    fn get_value(&self) -> u128 {
        let mut val = 0_u128;
        for byte_pos  in 0_usize..(self.size as usize) {
            unsafe {
                val += self.__bindgen_anon_1.value[byte_pos] as u128;
            }
            val <<= 8; 
        }
        val
    }
}
impl Debug for rr_GdbRegisterValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        unsafe {
        f.debug_struct("GdbRegisterValue")
            .field("name", &self.name)
            .field("size", &self.size)
            .field("defined", &self.defined)
            .field("value", &self.__bindgen_anon_1.value)
            .field("value_u128", &self.get_value())
            .finish()
        }    
    }
}
