#![windows_subsystem = "windows"]
#[warn(improper_ctypes)]
// extern crate libloading as lib;
use std::ffi::{CStr, CString};

use rustvcl::enums::*;
use rustvcl::funcs::*;
use rustvcl::imports::*;
use rustvcl::vcl::*;

// 按钮1单击事件
fn on_btn_click(_sender: usize) {
    ShowMessage("Hello, Rust!");
}

fn on_drop_file_event(_sender: usize, file_names: usize, len: isize) {
    println!("{}, {}, {}", _sender, file_names, len);
    unsafe {
        for i in 0..len {
            let file_name = CStr::from_ptr(DGetStringArrOf(file_names, i));
            println!("{}: {:?}", i, &file_name);
        }
    }
}

fn main() {
    // 乱写的，也不知道是不是这样写

    let app = TApplication::new();
    app.SetMainFormOnTaskBar(true);
    app.SetTitle("LCL App");
    app.Initialize();

    app.Icon().LoadFromFile("applogo.ico");

    let form = app.CreateForm();
    form.SetCaption("你好，Rust！ - Hello Rust!");
    form.SetPosition(TPosition::PoScreenCenter);
    form.SetAllowDropFiles(true);
    form.SetOnDropFiles(on_drop_file_event);
    form.SetOnClick(on_btn_click);

    let btn = TButton::new(TComponent::new_from_instance(form.Instance()));
    // btn.SetParent(form);
    // btn.SetLeft(10);
    // btn.SetTop(50);
    // btn.SetCaption("button1");
    // btn.SetOnClick(on_btn_click);

    app.Run();

    unsafe {

        // app实例
        // let app = Application_Instance();
        //
        // // 初始app
        //
        // Application_SetMainFormOnTaskBar(app, true);
        // Application_SetTitle(app, CString::new("LCL Form").unwrap().as_ptr());
        // Application_Initialize(app);
        // Application_SetMainFormOnTaskBar(app, true);
        // // 获取icon对象
        // let icon = Application_GetIcon(app);
        // // 从文件加载app图标
        // Icon_LoadFromFile(icon, CString::new("applogo.ico").unwrap().as_ptr());
        // // 创建一个form
        // let form = Application_CreateForm(app, false);
        //
        // // ResFormLoadFromFile(CString::new("./Form1.gfm").unwrap().as_ptr(), form);
        //
        // // 动态创建
        // // 设置form位置为屏幕中心
        // Form_SetPosition(form, TPosition::PoScreenCenter);
        //
        // // 设置form标题
        // Form_SetCaption(
        //     form,
        //     CString::new("你好，Rust！ - Hello Rust!").unwrap().as_ptr(),
        // );
        //
        // // 设置接受拖放文件
        // Form_SetAllowDropFiles(form, true);
        // Form_SetOnDropFiles(form, on_drop_file_event);
        //
        // // 创建按钮
        // let btn = Button_Create(form);
        // Button_SetParent(btn, form);
        // Button_SetLeft(btn, 10);
        // Button_SetTop(btn, 50);
        // Button_SetCaption(btn, CString::new("button1").unwrap().as_ptr());
        // Button_SetOnClick(btn, on_btn_click);
        //
        // // 运行app
        // Application_Run(app);
    }
}

// 动态加载测试

// fn main() {
//     load_lib_lcl();
//     println!("Hello, world!");
//     Test1("aaa", 1, 2);
// }
//
// fn Test1(str: &str, a: usize, b: isize) {
//     println!("{} = {} * {} = {}", str, a, b, a * b as usize);
// }

// fn load_lib_lcl() {
//     let lcl = lib::Library::new("liblcl.dll").unwrap();
//     unsafe {
//         let Application_Instance: lib::Symbol<unsafe extern "system" fn() -> usize> =
//             lcl.get(b"Application_Instance").unwrap();
//         let Application_Initialize: lib::Symbol<unsafe extern "system" fn(usize)> =
//             lcl.get(b"Application_Initialize").unwrap();
//         let Application_SetMainFormOnTaskBar: lib::Symbol<unsafe extern "system" fn(usize, bool)> =
//             lcl.get(b"Application_SetMainFormOnTaskBar").unwrap();
//         let Application_CreateForm: lib::Symbol<unsafe extern "system" fn(usize, bool) -> usize> =
//             lcl.get(b"Application_CreateForm").unwrap();
//         let Application_Run: lib::Symbol<unsafe extern "system" fn(usize)> =
//             lcl.get(b"Application_Run").unwrap();
//         let Form_SetPosition: lib::Symbol<unsafe extern "system" fn(usize, i32)> =
//             lcl.get(b"Form_SetPosition").unwrap();
//         let Form_SetCaption: lib::Symbol<unsafe extern "system" fn(usize, *const c_char)> =
//             lcl.get(b"Form_SetCaption").unwrap();
//
//         let Application_GetIcon: lib::Symbol<unsafe extern "system" fn(usize) -> usize> =
//             lcl.get(b"Application_GetIcon").unwrap();
//         let Icon_LoadFromFile: lib::Symbol<unsafe extern "system" fn(usize, *const c_char)> =
//             lcl.get(b"Icon_LoadFromFile").unwrap();
//
//         let PoScreenCenter = 4;
//
//         // app实例
//         let app = Application_Instance();
//         // 初始app
//
//         Application_Initialize(app);
//         Application_SetMainFormOnTaskBar(app, true);
//         // 获取icon对象
//         let icon = Application_GetIcon(app);
//         // 从文件加载app图标
//         Icon_LoadFromFile(icon, CString::new("app.ico").unwrap().as_ptr());
//         // 创建一个form
//         let form = Application_CreateForm(app, false);
//         // 设置form位置为屏幕中心
//         Form_SetPosition(form, PoScreenCenter);
//         // 设置form标题
//         Form_SetCaption(
//             form,
//             CString::new("你好，Rust！ - Hello Rust!").unwrap().as_ptr(),
//         );
//         // 运行app
//         Application_Run(app);
//     }
//     lcl.close().unwrap();
// }
