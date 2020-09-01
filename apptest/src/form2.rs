#![allow(non_snake_case)]
#![allow(dead_code)]

use rust_vcl::fns::*;
use rust_vcl::types::*;
use rust_vcl::vcl::*;

#[derive(VclForm)]
pub struct TForm2 {
    btn: TButton,
    pub form: TForm, // 固定名form, 放最后，前面引用完后，后面move到form。
}

impl TForm2 {
    pub fn new() -> Self {
        let form = Application.CreateForm();
        return Self {
            btn: NewObject!(TButton, form),
            form,
        };
    }

    pub fn init(&self) -> &Self {
     
        // TForm
        self.form
            .SetCaption("你好，Rust！ - Hello Rust!")
            .SetWidth(500)
            .SetHeight(300)
            .EnabledMaximize(false)
            .SetPosition(TPosition::poScreenCenter);

        // TButton
        self.btn.SetParent(self)
            .SetCaption("msgbox")
            .SetOnClick(self, Self::onBtnClick)
            .AnchorHorizontalCenterTo(self)
            .AnchorVerticalCenterTo(self);

        return self;
    }

    pub fn onBtnClick(&self, _sender: usize) {
        ShowMessage("form2");
    }
}
