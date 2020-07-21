#![windows_subsystem = "windows"]
#![allow(non_snake_case)]
#![allow(dead_code)]

// #[cfg(target_os = "windows")]

use rust_vcl::fns::*;
use rust_vcl::types::*;
use rust_vcl::vcl::*;

// 按钮1单击事件
fn onBtnClick(sender: usize) {
    ShowMessage("Hello, Rust! 你好，世界！");
    let btn = TButton::As(sender);
    println!("caption: {:?}", btn.Caption());
}

fn onDropFile(_sender: usize, fileNames: usize, len: isize) {
    println!("{}, {}, {}", _sender, fileNames, len);
    for i in 0..len {
        let fileName = GetFPStringArrayMember(fileNames, i);
        println!("{}: {:?}", i, &fileName);
    }
}

fn main() {
    // 乱写的，也不知道是不是这样写
    // 这里因为不会写，所以就这样弄下

    Application.SetMainFormOnTaskBar(true);
    Application.SetTitle("LCL App");
    Application.Initialize();
    Application.Icon().LoadFromFile("applogo.ico");

    let form = Application.CreateForm();
    form.SetCaption("你好，Rust！ - Hello Rust!");
    form.SetPosition(TPosition::poScreenCenter);
    form.SetAllowDropFiles(true);
    form.SetOnDropFiles(onDropFile);
    // form.SetOnClick(onBtnClick);

    // 测试自动drop
    // let ico = TIcon::new();
    //println!("{:?}", ico.ClassName());

    let btn = TButton::new(&form);
    btn.SetParent(&form);
    btn.SetLeft(10);
    btn.SetTop(50);
    btn.SetCaption("button1");
    btn.SetOnClick(onBtnClick);

    Application.Run();
}
