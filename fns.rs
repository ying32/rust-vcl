use crate::imports::*;
use std::ffi::{CStr, CString};

// 显示消息
pub fn ShowMessage(s: &str) {
    unsafe {
        DShowMessage(CString::new(s).unwrap().as_ptr());
    }
}

pub fn GetStringArrOf<'a>(ptr: usize, index: isize) -> &'a CStr {
    unsafe {
        return CStr::from_ptr(DGetStringArrOf(ptr, index));
    }
}
