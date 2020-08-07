#![allow(non_snake_case)]
#![allow(dead_code)]

use rust_vcl::fns::*;
use rust_vcl::types::*;
use rust_vcl::vcl::*;

 
pub struct TForm2 {
    btn: TButton,
    pub form: TForm, // 固定名form, 放最后，前面引用完后，后面move到form。
}

ImplForm!(TForm2);

impl TForm2 {
    pub fn new() -> Self {
        let form = Application.CreateForm();
        return Self {
            btn: NewObject!(TButton, form),
            form: form,
        };
    }

    pub fn init(&self) {

        let sid: usize = self.getSId();

        // TForm
        self.form.SetCaption("你好，Rust！ - Hello Rust!");
        self.form.SetWidth(200);
        self.form.SetHeight(300);
        self.form.EnabledMaximize(false);
        self.form.SetPosition(TPosition::poScreenCenter);

        // TButton
        self.btn.SetParent(self);
        self.btn.SetCaption("msgbox");
        self.btn.SetOnClick(sid, Self::onBtnClick);
        self.btn.AnchorHorizontalCenterTo(self);
        self.btn.AnchorVerticalCenterTo(self);
    }

    pub fn onBtnClick(&self, _sender: usize) {
        ShowMessage("form2");
    }

}