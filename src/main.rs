#![windows_subsystem = "windows"]

extern crate libloading as lib;

use std::ffi::CString;
use std::os::raw::c_char;

// libloading = "0.6.2"

// #[link(name = "liblcl")]
// extern "C" {
//     fn Application_Instance() -> uintptr_t;
// }

fn main() {
    load_lib_lcl();
    println!("Hello, world!");
    Test1("aaa", 1, 2);
}

fn Test1(str: &str, a: usize, b: isize) {
    println!("{} = {} * {} = {}", str, a, b, a * b as usize);
}

// pub struct Application {
//     instance: usize,
// }
//
// impl Application {
//     // fn _new() -> Result<Application, > {
//     //
//     // }
//     //
// }

fn load_lib_lcl() {
    let lcl = lib::Library::new("liblcl.dll").unwrap();
    unsafe {
        let Application_Instance: lib::Symbol<unsafe extern "C" fn() -> usize> =
            lcl.get(b"Application_Instance").unwrap();
        let Application_Initialize: lib::Symbol<unsafe extern "C" fn(usize)> =
            lcl.get(b"Application_Initialize").unwrap();
        let Application_SetMainFormOnTaskBar: lib::Symbol<unsafe extern "C" fn(usize, bool)> =
            lcl.get(b"Application_SetMainFormOnTaskBar").unwrap();
        let Application_CreateForm: lib::Symbol<unsafe extern "C" fn(usize, bool) -> usize> =
            lcl.get(b"Application_CreateForm").unwrap();
        let Application_Run: lib::Symbol<unsafe extern "C" fn(usize)> =
            lcl.get(b"Application_Run").unwrap();
        let Form_SetPosition: lib::Symbol<unsafe extern "C" fn(usize, i32)> =
            lcl.get(b"Form_SetPosition").unwrap();
        let Form_SetCaption: lib::Symbol<unsafe extern "C" fn(usize, *const c_char)> =
            lcl.get(b"Form_SetCaption").unwrap();

        let Application_GetIcon: lib::Symbol<unsafe extern "C" fn(usize) -> usize> =
            lcl.get(b"Application_GetIcon").unwrap();
        let Icon_LoadFromFile: lib::Symbol<unsafe extern "C" fn(usize, *const c_char)> =
            lcl.get(b"Icon_LoadFromFile").unwrap();

        let PoScreenCenter = 4;

        // app实例
        let app = Application_Instance();
        // 初始app
        Application_Initialize(app);
        Application_SetMainFormOnTaskBar(app, true);
        // 获取icon对象
        let icon = Application_GetIcon(app);
        // 从文件加载app图标
        Icon_LoadFromFile(icon, CString::new("app.ico").unwrap().as_ptr());
        // 创建一个form
        let form = Application_CreateForm(app, false);
        // 设置form位置为屏幕中心
        Form_SetPosition(form, PoScreenCenter);
        // 设置form标题
        Form_SetCaption(
            form,
            CString::new("你好，Rust！ - Hello Rust!").unwrap().as_ptr(),
        );
        // 运行app
        Application_Run(app);
    }
    lcl.close().unwrap();
}
