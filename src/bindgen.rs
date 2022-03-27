pub mod gdbconnection {
    include!(concat!(env!("OUT_DIR"), "/gdbconnection-bindings.rs"));
}
pub mod taskishuid {
    include!(concat!(env!("OUT_DIR"), "/taskishuid-bindings.rs"));
}
