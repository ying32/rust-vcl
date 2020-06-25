use crate::imports::*;
use std::ffi::CString;

// 显示消息
pub fn ShowMessage(s: &str) {
    unsafe {
        DShowMessage(CString::new(s).unwrap().as_ptr());
    }
}
