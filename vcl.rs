use crate::enums::*;
use crate::imports::*;
use crate::types::*;
use libc::{c_long, intptr_t, uintptr_t};
use std::ffi::CString;

pub struct TObject {
    instance: uintptr_t,
}

pub struct TIcon {
    instance: uintptr_t,
}

impl TIcon {
    pub fn new_instance(inst: uintptr_t) -> Self {
        TIcon { instance: inst }
    }

    pub fn LoadFromFile(&self, file_name: &str) {
        unsafe {
            Icon_LoadFromFile(self.instance, CString::new(file_name).unwrap().as_ptr());
        }
    }
}

// --------------------------

pub struct TForm {
    instance: uintptr_t,
}

impl TForm {
    // pub fn new(owner: uintptr_t) -> Self {
    //     TForm {
    //         instance: unsafe { Form_Create(owner) },
    //     }
    // }

    pub fn new_instance(inst: uintptr_t) -> Self {
        TForm { instance: inst }
    }

    pub fn SetCaption(&self, str: &str) {
        unsafe { Form_SetCaption(self.instance, CString::new(str).unwrap().as_ptr()) }
    }

    pub fn SetPosition(&self, val: TPosition) {
        unsafe { Form_SetPosition(self.instance, val) }
    }

    pub fn SetAllowDropFiles(&self, allow: bool) {
        unsafe {
            Form_SetAllowDropFiles(self.instance, allow);
        }
    }

    pub fn SetOnDropFiles(&self, event: TDropFilesEvent) {
        unsafe {
            Form_SetOnDropFiles(self.instance, event);
        }
    }

    pub fn SetOnClick(&self, event: TNotifyEvent) {
        unsafe {
            Form_SetOnClick(self.instance, event);
        }
    }
}

//-------------------

pub struct TApplication {
    instance: uintptr_t,
}

impl TApplication {
    pub fn new() -> Self {
        init_lib_lcl();
        TApplication {
            instance: unsafe { Application_Instance() },
        }
    }

    pub fn Initialize(&self) {
        unsafe {
            Application_Initialize(self.instance);
        }
    }

    pub fn SetMainFormOnTaskBar(&self, value: bool) {
        unsafe {
            Application_SetMainFormOnTaskBar(self.instance, value);
        }
    }

    pub fn SetTitle(&self, str: &str) {
        unsafe {
            Application_SetTitle(self.instance, CString::new(str).unwrap().as_ptr());
        }
    }

    pub fn Run(&self) {
        unsafe {
            Application_Run(self.instance);
        }
    }

    pub fn CreateForm(&self) -> TForm {
        TForm::new_instance(unsafe { Application_CreateForm(self.instance, false) })
    }

    pub fn Icon(&self) -> TIcon {
        TIcon::new_instance(unsafe { Application_GetIcon(self.instance) })
    }
}

//--------------

pub struct TButton {
    instance: uintptr_t,
}

impl TButton {
    pub fn new(owner: uintptr_t) -> Self {
        TButton {
            instance: unsafe { Button_Create(owner) },
        }
    }

    pub fn SetParent(&self, parent: uintptr_t) {
        unsafe {
            Button_SetParent(self.instance, parent);
        }
    }

    pub fn SetLeft(&self, value: i32) {
        unsafe {
            Button_SetLeft(self.instance, value);
        }
    }

    pub fn SetTop(&self, val: i32) {
        unsafe {
            Button_SetTop(self.instance, val);
        }
    }

    pub fn SetCaption(&self, str: &str) {
        unsafe {
            Button_SetCaption(self.instance, CString::new(str).unwrap().as_ptr());
        }
    }

    pub fn SetOnClick(&self, event: TNotifyEvent) {
        unsafe {
            Button_SetOnClick(self.instance, event);
        }
    }
}
