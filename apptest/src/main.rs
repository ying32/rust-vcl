//#![windows_subsystem = "windows"]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_use]
extern crate rust_vcl;

mod form2;
mod mainform;

use rust_vcl::fns::*;
use rust_vcl::vcl::*;

use form2::TForm2;
use mainform::TMainForm;

// const array:[u8;4] = [1,2,3,3];

fn test() {
    // async_std::task::spawn(async {
    //     println!("async task");
    //     a.await;
    // });

    let guid = CreateGUID();
    println!("{}-{}-{}-{:?}", guid.d1, guid.d2, guid.d3, guid.d4);
    println!("{:}", GUIDToString(&guid));
    println!("{:}", LibAbout());
    let nullptr = 0 as *const i8;
    let abc = ToRustString(nullptr);
    println!("abc={}", &abc);
}

#[derive(VclApp)]
struct TApp {
    mainForm: TMainForm,
    form2: TForm2,
}

impl TApp {
    fn new() -> Self {
        Application.SetMainFormOnTaskBar(true);
        Application.SetTitle("LCL App");
        Application.Initialize();
        // Windows上，自动加载exe中名为MAINICON的图标。
        //Application.Icon().LoadFromFile("applogo.ico");

        return Self {
            mainForm: TMainForm::new(),
            form2: TForm2::new(),
        };
    }

    fn init(&self) -> &Self {
        // ????
        //Application.SetOnException(self.getSId(), Self::onException);

        println!("currentthreadid={}", CurrentThreadId());
        self.mainForm
            .init()
            .btnOpenForm2
            .SetOnClick(self, Self::onOpenForm2);

        self.form2.init();

        return self;
    }

    fn onOpenForm2(&self, _sender: usize) {
        self.form2.form.Show();
    }

    fn onException(&self, _sender: usize, e: usize) {
        let exception = Exception::As(e);
        println!("exception: {:}", exception.Message());
    }
}

fn main() {
    test();
    // 写gui的感觉不太方便。唉。。。。
    // 现在的一切都是实验，啥东西感觉随时会变动，随着我对Rust的熟悉成度不断变化，直到满意。
    println!("threadid={}", MainThreadId());
    println!("currentthreadid={}", CurrentThreadId());
    let app = TApp::new();
    app.init().run();
}
