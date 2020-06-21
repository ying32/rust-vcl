#![windows_subsystem = "windows"]
#[warn(improper_ctypes)]
// extern crate libloading as lib;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern crate libc;
use libc::{c_long, intptr_t, uintptr_t};
use std::mem::{size_of, transmute};

// libloading = "0.6.2"

#[repr(C)]
enum TPosition {
    PoDesigned,        // use bounds from the designer (read from stream)
    PoDefault,         // LCL decision (normally window manager decides)
    PoDefaultPosOnly,  // designed size and LCL position
    PoDefaultSizeOnly, // designed position and LCL size
    PoScreenCenter,    // center form on screen (depends on DefaultMonitor)
    PoDesktopCenter,   // center form on desktop (total of all screens)
    PoMainFormCenter,  // center form on main form (depends on DefaultMonitor)
    PoOwnerFormCenter, // center form on owner form (depends on DefaultMonitor)
    PoWorkAreaCenter,  // center form on working area (depends on DefaultMonitor)
}

type TNotifyEvent = fn(uintptr_t);
type TDropFilesEvent = fn(uintptr_t, uintptr_t, intptr_t);

// MSVC 编译器，静态加载
#[link(name = "liblcl")]
extern "system" {

    fn Application_Instance() -> uintptr_t;
    fn Application_Initialize(obj: uintptr_t);
    fn Application_SetTitle(obj: uintptr_t, val: *const c_char);
    fn Application_SetMainFormOnTaskBar(obj: uintptr_t, val: bool);
    fn Application_CreateForm(obj: uintptr_t, init_scale: bool) -> uintptr_t;
    fn Application_Run(obj: uintptr_t);
    fn Form_SetPosition(obj: uintptr_t, val: TPosition);
    fn Form_SetCaption(obj: uintptr_t, val: *const c_char);
    fn Application_GetIcon(obj: uintptr_t) -> uintptr_t;
    fn Icon_LoadFromFile(obj: uintptr_t, file_name: *const c_char);

    //TButton
    fn Button_Create(owner: uintptr_t) -> uintptr_t;
    fn Button_SetCaption(obj: uintptr_t, val: *const c_char);
    fn Button_SetParent(obj: uintptr_t, parent: uintptr_t);
    fn Button_SetOnClick(obj: uintptr_t, event_id: TNotifyEvent);
    fn Button_SetLeft(obj: uintptr_t, val: i32);
    fn Button_SetTop(obj: uintptr_t, val: i32);

    fn Object_ToString(obj: uintptr_t) -> *const c_char;

    fn Icon_SetHandle(obj: uintptr_t, hints: uintptr_t, name: *const c_char);

    fn Form_SetAllowDropFiles(obj: uintptr_t, allow: bool);
    fn Form_SetOnDropFiles(obj: uintptr_t, event: TDropFilesEvent);

    fn DGetStringArrOf(ptr: uintptr_t, index: intptr_t) -> *const c_char;

    //
    fn DShowMessage(msg: *const c_char);
    fn SetEventCallback(
        callback: extern "system" fn(f: uintptr_t, args: uintptr_t, arg_count: c_long) -> uintptr_t,
    );
}

// 根据索引获取参数
unsafe fn get_param_of(index: uintptr_t, ptr: uintptr_t) -> uintptr_t {
    return *((ptr + index * size_of::<uintptr_t>()) as *const uintptr_t);
}

// 回调函数
extern "system" fn do_event_callback_proc(
    f: uintptr_t,
    args: uintptr_t,
    arg_count: c_long,
) -> uintptr_t {
    println!("do_event_callback_proc=({}, {}, {})", f, args, arg_count);
    unsafe {
        match arg_count {
            0 => transmute::<uintptr_t, fn()>(f)(),
            1 => transmute::<uintptr_t, fn(uintptr_t)>(f)(get_param_of(0, args)),
            2 => transmute::<uintptr_t, fn(uintptr_t, uintptr_t)>(f)(
                get_param_of(0, args),
                get_param_of(1, args),
            ),
            3 => transmute::<uintptr_t, fn(uintptr_t, uintptr_t, uintptr_t)>(f)(
                get_param_of(0, args),
                get_param_of(1, args),
                get_param_of(2, args),
            ),
            4 => transmute::<uintptr_t, fn(uintptr_t, uintptr_t, uintptr_t, uintptr_t)>(f)(
                get_param_of(0, args),
                get_param_of(1, args),
                get_param_of(2, args),
                get_param_of(3, args),
            ),
            5 => {
                transmute::<uintptr_t, fn(uintptr_t, uintptr_t, uintptr_t, uintptr_t, uintptr_t)>(f)(
                    get_param_of(0, args),
                    get_param_of(1, args),
                    get_param_of(2, args),
                    get_param_of(3, args),
                    get_param_of(4, args),
                )
            }
            // 最多12个参数，这里只是演示，所以只实现5个参数的，多的自己实现吧
            // 6=>,
            // 7=>,
            // 8=>,
            // 9=>,
            // 10=>,
            // 11=>,
            // 12=>,
            _ => println!("none"),
        }
    }
    return 0;
}

fn ShowMessage(s: &str) {
    unsafe {
        DShowMessage(CString::new(s).unwrap().as_ptr());
    }
}

// 按钮1单击事件
fn on_btn_click(_sender: uintptr_t) {
    ShowMessage("Hello, Rust!");
}

fn on_drop_file_event(_sender: uintptr_t, file_names: uintptr_t, len: intptr_t) {
    println!("{}, {}, {}", _sender, file_names, len);
    unsafe {
        for i in 0..len {
            let file_name = CStr::from_ptr(DGetStringArrOf(file_names, i));
            println!("{}: {:?}", i, &file_name);
        }
    }
}

fn main() {
    unsafe {
        // 基本事件回调
        SetEventCallback(do_event_callback_proc);

        // app实例
        let app = Application_Instance();

        // 初始app

        Application_SetMainFormOnTaskBar(app, true);
        Application_SetTitle(app, CString::new("LCL Form").unwrap().as_ptr());
        Application_Initialize(app);
        Application_SetMainFormOnTaskBar(app, true);
        // 获取icon对象
        let icon = Application_GetIcon(app);
        // 从文件加载app图标
        Icon_LoadFromFile(icon, CString::new("applogo.ico").unwrap().as_ptr());
        // 创建一个form
        let form = Application_CreateForm(app, false);
        // 设置form位置为屏幕中心
        Form_SetPosition(form, TPosition::PoScreenCenter);
        // 设置form标题
        Form_SetCaption(
            form,
            CString::new("你好，Rust！ - Hello Rust!").unwrap().as_ptr(),
        );

        // 设置接受拖放文件
        Form_SetAllowDropFiles(form, true);
        Form_SetOnDropFiles(form, on_drop_file_event);

        // 创建按钮
        let btn = Button_Create(form);
        Button_SetParent(btn, form);
        Button_SetLeft(btn, 10);
        Button_SetTop(btn, 50);
        Button_SetCaption(btn, CString::new("button1").unwrap().as_ptr());
        Button_SetOnClick(btn, on_btn_click);

        // 运行app
        Application_Run(app);
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
