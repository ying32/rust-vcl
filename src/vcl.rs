#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::imports::*;
use crate::types::*;

use std::ffi::CString;

pub trait IObject {
    fn Instance(&self) -> usize;
}

pub trait IComponent: IObject {
    fn Name<'a>(&self) -> &str;
}

pub trait IControl: IComponent {}

pub trait IWinControl: IControl {}

//-------------------------------------------
pub struct TObject(usize);

impl IObject for TObject {
    fn Instance(&self) -> usize {
        unimplemented!()
    }
}

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
    pub fn new(owner: &dyn IComponent) -> Self {
        TComponent {
            0: owner.Instance(),
        }
    }

    pub fn new_from_instance(inst: usize) -> Self {
        TComponent { 0: inst }
    }
}

impl IObject for TComponent {
    fn Instance(&self) -> usize {
        return self.0;
    }
}

impl IComponent for TComponent {
    fn Name<'a>(&self) -> &str {
        unimplemented!()
    }
}

//------------------------------------------------

pub struct TControl(usize);

impl IObject for TControl {
    fn Instance(&self) -> usize {
        return self.0;
    }
}

impl IComponent for TControl {
    fn Name<'a>(&self) -> &str {
        unimplemented!()
    }
}
//------------------------------------------------

pub struct TWinControl(usize);

impl TWinControl {
    pub fn new(owner: &dyn IComponent) -> Self {
        TWinControl {
            0: owner.Instance(),
        }
    }

    pub fn new_from_instance(inst: usize) -> Self {
        TWinControl { 0: inst }
    }
}

impl IObject for TWinControl {
    fn Instance(&self) -> usize {
        return self.0;
    }
}

impl IComponent for TWinControl {
    fn Name<'a>(&self) -> &str {
        unimplemented!()
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
}

impl IObject for TIcon {
    fn Instance(&self) -> usize {
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

impl IObject for TForm {
    fn Instance(&self) -> usize {
        return self.0;
    }
}

impl IComponent for TForm {
    fn Name<'a>(&self) -> &str {
        unimplemented!()
    }
}

impl IControl for TForm {}

impl IWinControl for TForm {}

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

impl IObject for TApplication {
    fn Instance(&self) -> usize {
        return self.0;
    }
}

impl IComponent for TApplication {
    fn Name<'a>(&self) -> &str {
        unimplemented!()
    }
}

//--------------

pub struct TButton(usize);

impl TButton {
    pub fn new(owner: &dyn IComponent) -> Self {
        TButton {
            0: unsafe { Button_Create(owner.Instance()) },
        }
    }

    pub fn SetParent(&self, parent: &dyn IWinControl) {
        unsafe {
            Button_SetParent(self.0, parent.Instance());
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

impl IObject for TButton {
    fn Instance(&self) -> usize {
        return self.0;
    }
}

impl IComponent for TButton {
    fn Name<'a>(&self) -> &str {
        unimplemented!()
    }
}

impl IControl for TButton {}

impl IWinControl for TButton {}
//------------------------------------全局---------------------------------------------------------
lazy_static! {
    pub static ref Application: TApplication = TApplication::new();
}
