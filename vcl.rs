use crate::enums::*;
use crate::imports::*;
use crate::types::*;

use std::ffi::CString;

pub struct TObject {
    instance: usize,
}

pub struct TComponent {
    instance: usize,
}

impl TComponent {
    pub fn new(owner: &TComponent) -> Self {
        TComponent { instance: 0 }
    }

    pub fn new_from_instance(inst: usize) -> Self {
        TComponent { instance: inst }
    }

    pub fn Instance(&self) -> usize {
        return self.instance;
    }
}

pub struct TControl {
    instance: usize,
}

pub struct TWinControl {
    instance: usize,
}

pub struct TIcon {
    instance: usize,
}

impl TIcon {
    pub fn new_from_instance(inst: usize) -> Self {
        TIcon { instance: inst }
    }

    pub fn LoadFromFile(&self, file_name: &str) {
        unsafe {
            Icon_LoadFromFile(self.instance, CString::new(file_name).unwrap().as_ptr());
        }
    }

    pub fn Instance(&self) -> usize {
        return self.instance;
    }
}

// --------------------------

pub struct TForm {
    instance: usize,
}

impl TForm {
    pub fn new(owner: TComponent) -> Self {
        TForm {
            instance: unsafe { Form_Create(owner.instance) },
        }
    }

    pub fn new_from_instance(inst: usize) -> Self {
        TForm { instance: inst }
    }

    pub fn Instance(&self) -> usize {
        return self.instance;
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
    instance: usize,
}

impl TApplication {
    pub fn new() -> Self {
        init_lib_lcl();
        TApplication {
            instance: unsafe { Application_Instance() },
        }
    }

    pub fn Instance(&self) -> usize {
        return self.instance;
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
        TForm::new_from_instance(unsafe { Application_CreateForm(self.instance, false) })
    }

    pub fn Icon(&self) -> TIcon {
        TIcon::new_from_instance(unsafe { Application_GetIcon(self.instance) })
    }
}

//--------------

pub struct TButton {
    instance: usize,
}

impl TButton {
    pub fn new(owner: TComponent) -> Self {
        TButton {
            instance: unsafe { Button_Create(owner.instance) },
        }
    }

    pub fn Instance(&self) -> usize {
        return self.instance;
    }

    pub fn SetParent(&self, parent: usize) {
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
