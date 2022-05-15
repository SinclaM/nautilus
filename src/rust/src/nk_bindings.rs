// without these, cargo complains loudly
// about many C constants and functions
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// include generated bindings in this file
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
