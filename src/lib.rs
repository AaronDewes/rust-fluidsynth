extern crate libc;
use std::ffi::CString;
#[allow(
    dead_code,
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case
)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/ffi.rs"));
}

pub mod audio;
pub mod event;
pub mod gen;
pub mod log;
pub mod midi;
pub mod modulator;
pub mod seq;
pub mod settings;
pub mod sfont;
pub mod synth;

pub fn is_soundfont(filename: &str) -> bool {
    let name = CString::new(filename).unwrap();
    unsafe { ffi::fluid_is_soundfont(name.as_ptr()) != 0 }
}

pub fn is_midifile(filename: &str) -> bool {
    let name = CString::new(filename).unwrap();
    unsafe { ffi::fluid_is_midifile(name.as_ptr()) != 0 }
}
