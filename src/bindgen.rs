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
pub use crate::bindgen::gdbconnection::rr_GdbRegisterValue;
pub use crate::bindgen::gdbconnection::rr_GdbThreadId_ANY;
pub use crate::bindgen::gdbconnection::rr_GdbThreadId_ALL;

impl rr_GdbRegisterValue{
    pub fn get_value_u128(&self) -> u128 {
        let mut val = 0_u128;
        for byte_pos_invert  in 0_usize..(self.size as usize) {
            let pos = self.size as usize - byte_pos_invert-1;
            val <<= 8; 
            // TODO : Missing SAFETY
            unsafe {
                val += self.__bindgen_anon_1.value[pos] as u128;
            }
        }
        val
    }
}
impl Debug for rr_GdbRegisterValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // TODO : Missing SAFETY
        unsafe {
        f.debug_struct("GdbRegisterValue")
            .field("name", &self.name)
            .field("size", &self.size)
            .field("defined", &self.defined)
            .field("value", &self.__bindgen_anon_1.value)
            .field("value_u128", &self.get_value_u128())
            .finish()
        }    
    }
}
