#![warn(improper_ctypes)]

// use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::enums::*;
use crate::event_callback::do_event_callback_proc;
use crate::types::*;

// MSVC 编译器，静态加载
#[link(name = "liblcl")]
extern "system" {

    pub fn Application_Instance() -> usize;
    pub fn Application_Initialize(obj: usize);
    pub fn Application_SetTitle(obj: usize, val: *const c_char);
    pub fn Application_SetMainFormOnTaskBar(obj: usize, val: bool);
    pub fn Application_CreateForm(obj: usize, init_scale: bool) -> usize;
    pub fn Application_Run(obj: usize);
    pub fn Form_SetPosition(obj: usize, val: TPosition);
    pub fn Form_SetCaption(obj: usize, val: *const c_char);
    pub fn Application_GetIcon(obj: usize) -> usize;
    pub fn Icon_LoadFromFile(obj: usize, file_name: *const c_char);

    //TButton
    pub fn Button_Create(owner: usize) -> usize;
    pub fn Button_SetCaption(obj: usize, val: *const c_char);
    pub fn Button_SetParent(obj: usize, parent: usize);
    pub fn Button_SetOnClick(obj: usize, event_id: TNotifyEvent);
    pub fn Button_SetLeft(obj: usize, val: i32);
    pub fn Button_SetTop(obj: usize, val: i32);

    pub fn Object_ToString(obj: usize) -> *const c_char;

    pub fn Icon_SetHandle(obj: usize, hints: usize, name: *const c_char);

    pub fn Form_SetAllowDropFiles(obj: usize, allow: bool);
    pub fn Form_SetOnDropFiles(obj: usize, event: TDropFilesEvent);
    pub fn Form_SetOnClick(obj: usize, event: TNotifyEvent);
    pub fn Form_Create(owner: usize) -> usize;

    pub fn DGetStringArrOf(ptr: usize, index: isize) -> *const c_char;

    //
    pub fn ResFormLoadFromFile(file_name: *const c_char, root: usize);

    //
    pub fn DShowMessage(msg: *const c_char);
    fn SetEventCallback(
        callback: extern "system" fn(f: usize, args: usize, arg_count: i32) -> usize,
    );
}

pub fn init_lib_lcl() {
    unsafe {
        // 基本事件回调
        SetEventCallback(do_event_callback_proc);
    }
}
