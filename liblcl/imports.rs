use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::enums::*;
use crate::event_callback::do_event_callback_proc;
use crate::types::*;
use libc::{c_long, intptr_t, uintptr_t};
use std::mem::{size_of, transmute};

// MSVC 编译器，静态加载
#[link(name = "liblcl")]
extern "system" {

    pub fn Application_Instance() -> uintptr_t;
    pub fn Application_Initialize(obj: uintptr_t);
    pub fn Application_SetTitle(obj: uintptr_t, val: *const c_char);
    pub fn Application_SetMainFormOnTaskBar(obj: uintptr_t, val: bool);
    pub fn Application_CreateForm(obj: uintptr_t, init_scale: bool) -> uintptr_t;
    pub fn Application_Run(obj: uintptr_t);
    pub fn Form_SetPosition(obj: uintptr_t, val: TPosition);
    pub fn Form_SetCaption(obj: uintptr_t, val: *const c_char);
    pub fn Application_GetIcon(obj: uintptr_t) -> uintptr_t;
    pub fn Icon_LoadFromFile(obj: uintptr_t, file_name: *const c_char);

    //TButton
    pub fn Button_Create(owner: uintptr_t) -> uintptr_t;
    pub fn Button_SetCaption(obj: uintptr_t, val: *const c_char);
    pub fn Button_SetParent(obj: uintptr_t, parent: uintptr_t);
    pub fn Button_SetOnClick(obj: uintptr_t, event_id: TNotifyEvent);
    pub fn Button_SetLeft(obj: uintptr_t, val: i32);
    pub fn Button_SetTop(obj: uintptr_t, val: i32);

    pub fn Object_ToString(obj: uintptr_t) -> *const c_char;

    pub fn Icon_SetHandle(obj: uintptr_t, hints: uintptr_t, name: *const c_char);

    pub fn Form_SetAllowDropFiles(obj: uintptr_t, allow: bool);
    pub fn Form_SetOnDropFiles(obj: uintptr_t, event: TDropFilesEvent);

    pub fn DGetStringArrOf(ptr: uintptr_t, index: intptr_t) -> *const c_char;

    //
    pub fn ResFormLoadFromFile(file_name: *const c_char, root: uintptr_t);

    //
    pub fn DShowMessage(msg: *const c_char);
    fn SetEventCallback(
        callback: extern "system" fn(f: uintptr_t, args: uintptr_t, arg_count: c_long) -> uintptr_t,
    );
}

pub fn init_lib_lcl() {
    unsafe {
        // 基本事件回调
        SetEventCallback(do_event_callback_proc);
    }
}
