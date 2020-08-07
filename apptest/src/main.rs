//#![windows_subsystem = "windows"]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_use]
extern crate rust_vcl;

mod mainform;
mod form2;

use mainform::TMainForm;
use form2::TForm2;
use rust_vcl::fns::*;
use rust_vcl::vcl::*;


fn test() {
  
    let guid = CreateGUID();
    println!("{}-{}-{}-{:?}", guid.d1, guid.d2, guid.d3, guid.d4);
    println!("{:}", GUIDToString(&guid));
    println!("{:}", LibAbout());

    // let abc = TGridRect::Empty();
}

struct TApp {
   mainForm: TMainForm,
   form2: TForm2,
}

// 有没有隐式实现现的方式？？？
ImplISId!(TApp);

impl TApp {

    fn new() -> Self {

        Application.SetMainFormOnTaskBar(true);
        Application.SetTitle("LCL App");
        Application.Initialize();
        Application.Icon().LoadFromFile("applogo.ico");

        return Self {
            mainForm: TMainForm::new(),
            form2: TForm2::new(),
        };
    }

    fn init(&self) {
        self.mainForm.init();
        self.form2.init();
        self.mainForm.btnOpenForm2.SetOnClick(self.getSId(), Self::onOpenForm2)
    }

    fn run(&self) {
        Application.Run();
    }

    fn onOpenForm2(&self, _sender: usize) {
        self.form2.form.Show();
    }
}



fn main() {
    test();
    // 写gui的感觉不太方便。唉。。。。
    // 现在的一切都是实验，啥东西感觉随时会变动，随着我对Rust的熟悉成度不断变化，直到满意。
    let app = TApp::new();
    app.init();
    app.run();
}
