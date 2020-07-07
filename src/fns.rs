#![allow(non_snake_case)]

use crate::imports::*;
use std::borrow::Cow;
use std::ffi::{CStr, CString};

// 显示消息
pub fn ShowMessage(s: &str) {
    unsafe {
        DShowMessage(CString::new(s).unwrap().as_ptr());
    }
}

pub fn GetStringArrOf<'a>(ptr: usize, index: isize) -> Cow<'a, str> {
    unsafe {
        return CStr::from_ptr(DGetStringArrOf(ptr, index)).to_string_lossy();
    }
}
