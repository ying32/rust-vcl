extern crate libc;
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;

extern crate vcl_derives;
pub use vcl_derives::*;

pub mod fns;
pub mod lclapi;
pub mod types;
mod typesimpl;

pub mod vcl;
