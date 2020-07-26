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

fn onBtn2Click(_sender: usize) {
    //TStrings是虚方法类，当参数类型为TStrings要用TStringList传入，非TComponent实现了drop方法，所以Free不是必须的
    let list = TStringList::new();
    list.Add("Item1");
    list.Add("Item2");
    list.Add("Item3");
    let idx = InputCombo(
        "caption",
        "Prompt                          ", // aPrompt参数决定显示的窗口宽度
        &list,
    );
    println!("select index: {}", idx);
}

fn onDropFile(_sender: usize, fileNames: usize, len: isize) {
    println!("{}, {}, {}", _sender, fileNames, len);
    for i in 0..len {
        let fileName = GetFPStringArrayMember(fileNames, i);
        println!("{}: {:?}", i, &fileName);
    }
}

fn onFormMouseMove(_sender: usize, _shift: TShiftState, _x: i32, _y: i32) {
    if InSet(_shift, TShiftStateEnum::ssCtrl as u8) {
        println!("ctrl");
    }
    // let form = TForm::As(_sender);
    // let pos = Mouse.CursorPos();
    // println!("x={}, y={}, sx={}, sy={}", _x, _y, pos.x, pos.y);
}

fn test() {
    let guid = CreateGUID();
    println!("{}-{}-{}-{:?}", guid.d1, guid.d2, guid.d3, guid.d4);
    println!("{:?}", GUIDToString(&guid));
    println!("{:?}", LibAbout());

    // let abc = TGridRect::Empty();
}

fn main() {
    test();

    Application.SetMainFormOnTaskBar(true);
    Application.SetTitle("LCL App");
    Application.Initialize();
    Application.Icon().LoadFromFile("applogo.ico");

    let form = Application.CreateForm();
    form.SetCaption("你好，Rust！ - Hello Rust!");
    form.SetPosition(TPosition::poScreenCenter);
    form.SetAllowDropFiles(true);
    form.SetOnDropFiles(onDropFile);
    form.SetOnMouseMove(onFormMouseMove);
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

    let btn2 = TButton::new(&form);
    btn2.SetParent(&form);
    btn2.SetLeft(10);
    btn2.SetTop(btn.Top() + btn.Height() + 10);
    btn2.SetWidth(120);
    btn2.SetCaption("InputCombo");
    btn2.SetOnClick(onBtn2Click);

    Application.Run();
}
