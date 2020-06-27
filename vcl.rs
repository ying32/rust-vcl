#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::enums::*;
use crate::imports::*;
use crate::types::*;

use std::ffi::CString;

pub struct TObject(usize);

pub struct TMouse(usize);

pub struct TScreen(usize);

pub struct TPrinter(usize);

impl TPrinter {
    pub fn new() -> Self {
        TPrinter { 0: 0 }
    }

    pub fn new_from_instance(inst: usize) -> Self {
        TPrinter { 0: inst }
    }

    pub fn SetPrinter(&self) {}
}

//---------------------------------------------

pub struct TClipboard(usize);

//------------------------------------------------

pub struct TComponent(usize);

impl TComponent {
    pub fn new(owner: TComponent) -> Self {
        TComponent { 0: owner.0 }
    }

    pub fn new_from_instance(inst: usize) -> Self {
        TComponent { 0: inst }
    }

    pub fn Instance(&self) -> usize {
        return self.0;
    }
}

//------------------------------------------------

pub struct TControl(usize);

//------------------------------------------------

pub struct TWinControl(usize);

impl TWinControl {
    pub fn new(owner: TComponent) -> Self {
        TWinControl { 0: owner.0 }
    }

    pub fn new_from_instance(inst: usize) -> Self {
        TWinControl { 0: inst }
    }

    pub fn Instance(&self) -> usize {
        return self.0;
    }
}

//------------------------------------------------

pub struct TIcon(usize);

impl TIcon {
    pub fn new_from_instance(inst: usize) -> Self {
        TIcon { 0: inst }
    }

    pub fn LoadFromFile(&self, file_name: &str) {
        unsafe {
            Icon_LoadFromFile(self.0, CString::new(file_name).unwrap().as_ptr());
        }
    }

    pub fn Instance(&self) -> usize {
        return self.0;
    }
}

// --------------------------

pub struct TForm(usize);

impl TForm {
    pub fn new(owner: TComponent) -> Self {
        TForm {
            0: unsafe { Form_Create(owner.0) },
        }
    }

    pub fn new_from_instance(inst: usize) -> Self {
        TForm { 0: inst }
    }

    pub fn Instance(&self) -> usize {
        return self.0;
    }

    pub fn SetCaption(&self, str: &str) {
        unsafe { Form_SetCaption(self.0, CString::new(str).unwrap().as_ptr()) }
    }

    pub fn SetPosition(&self, val: TPosition) {
        unsafe { Form_SetPosition(self.0, val) }
    }

    pub fn SetAllowDropFiles(&self, allow: bool) {
        unsafe {
            Form_SetAllowDropFiles(self.0, allow);
        }
    }

    pub fn SetOnDropFiles(&self, event: TDropFilesEvent) {
        unsafe {
            Form_SetOnDropFiles(self.0, event);
        }
    }

    pub fn SetOnClick(&self, event: TNotifyEvent) {
        unsafe {
            Form_SetOnClick(self.0, event);
        }
    }
}

//-------------------

pub struct TApplication(usize);

impl TApplication {
    // 不公开这个，新建一个app实例，并初始化事件的东西
    // 全局静态
    fn new() -> Self {
        init_lib_lcl();
        TApplication {
            0: unsafe { Application_Instance() },
        }
    }

    pub fn Instance(&self) -> usize {
        return self.0;
    }

    pub fn Initialize(&self) {
        unsafe {
            Application_Initialize(self.0);
        }
    }

    pub fn SetMainFormOnTaskBar(&self, value: bool) {
        unsafe {
            Application_SetMainFormOnTaskBar(self.0, value);
        }
    }

    pub fn SetTitle(&self, str: &str) {
        unsafe {
            Application_SetTitle(self.0, CString::new(str).unwrap().as_ptr());
        }
    }

    pub fn Run(&self) {
        unsafe {
            Application_Run(self.0);
        }
    }

    pub fn CreateForm(&self) -> TForm {
        TForm::new_from_instance(unsafe { Application_CreateForm(self.0, false) })
    }

    pub fn Icon(&self) -> TIcon {
        TIcon::new_from_instance(unsafe { Application_GetIcon(self.0) })
    }
}

//--------------

pub struct TButton(usize);

impl TButton {
    pub fn new(owner: TComponent) -> Self {
        TButton {
            0: unsafe { Button_Create(owner.0) },
        }
    }

    pub fn Instance(&self) -> usize {
        return self.0;
    }

    pub fn SetParent(&self, parent: TWinControl) {
        unsafe {
            Button_SetParent(self.0, parent.0);
        }
    }

    pub fn SetLeft(&self, value: i32) {
        unsafe {
            Button_SetLeft(self.0, value);
        }
    }

    pub fn SetTop(&self, val: i32) {
        unsafe {
            Button_SetTop(self.0, val);
        }
    }

    pub fn SetCaption(&self, str: &str) {
        unsafe {
            Button_SetCaption(self.0, CString::new(str).unwrap().as_ptr());
        }
    }

    pub fn SetOnClick(&self, event: TNotifyEvent) {
        unsafe {
            Button_SetOnClick(self.0, event);
        }
    }
}

//------------------------------------全局---------------------------------------------------------
lazy_static! {
    pub static ref Application: TApplication = TApplication::new();
}
