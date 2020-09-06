//#![windows_subsystem = "windows"]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_use]
extern crate rust_vcl;

mod form2;
mod form3;
mod mainform;

use rust_vcl::fns::*;
use rust_vcl::vcl::*;

use form2::TForm2;
use form3::TForm3;
use mainform::TMainForm;

// const array:[u8;4] = [1,2,3,3];

struct abc1;

// impl abc1 {
//     fn new() -> Self {
//         abc1 {}
//     }

// }

// impl From<usize> for abc1 {
//     fn from(val: usize) -> Self {
//         println!("usize: {}", val);
//         abc1{}
//     }
// }

fn test() {
    // let abcdef = abc1::new();
    // abc1::from(111);

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
    form3: TForm3,
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
            form3: TForm3::new(),
        };
    }

    fn init(&self) -> &Self {
        // ????
        //Application.SetOnException(self.getSId(), Self::onException);

        println!("currentthreadid={}", CurrentThreadId());
        self.mainForm.init();

        self.mainForm
            .btnOpenForm2
            .SetOnClick(self, Self::onOpenForm2);

        self.mainForm
            .btnOpenForm3
            .SetOnClick(self, Self::onOpenForm3);

        self.form2.init();
        self.form3.init();

        return self;
    }

    fn onOpenForm2(&self, _sender: usize) {
        //self.form2.form.Show();
        self.form2.form.ShowModal();
    }

    fn onOpenForm3(&self, _sender: usize) {
        //self.form3.form.Show();
        self.form3.form.ShowModal();
    }

    fn onException(&self, _sender: usize, e: usize) {
        let exception = Exception::from(e);
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
