#![allow(non_snake_case)]
#![allow(dead_code)]

use rust_vcl::fns::*;
// use rust_vcl::types::*;
use rust_vcl::vcl::*;

#[derive(VclForm)]
pub struct TForm3 {
    pub form: TForm, // 固定名form, 放最后，前面引用完后，后面move到form。
}

impl TForm3 {
    pub fn new() -> Self {
        let form = Application.CreateForm();
        // 这里只做个演示
        #[cfg(all(target_env = "msvc", target_os = "windows"))]
        ResFormLoadFromResourceName(GetMainInstance(), "TFORM3", &form);
        // 资源FORM相关的API
        // ResFormLoadFromFile
        // ResFormLoadFromStream
        // ResFormRegisterFormResource
        // ResFormLoadFromClassName

        return Self {
            // 或者这里FindCompoent
            form,
        };
    }

    pub fn init(&self) -> &Self {
        // 这里简单演示下，随意的写个
        // say hello 按钮
        let comp = self.form.FindComponent("Button1");
        if !comp.IsNil() {
            let btn = TButton::into(&comp);
            btn.SetOnClick(self, Self::onBtnClick);
        }

        return self;
    }

    pub fn onBtnClick(&self, _sender: usize) {
        ShowMessage("form3: hello!");
    }
}
