#![allow(non_snake_case)]
#![allow(dead_code)]

use rust_vcl::fns::*;
use rust_vcl::types::*;
use rust_vcl::vcl::*;

#[derive(VclForm)]
pub struct TMainForm {
    btn: TButton,
    btn2: TButton,
    btn3: TButton,
    btnOpenDialog: TButton,
    btnColorDialog: TButton,
    dlgOpen: TOpenDialog,
    dlgColor: TColorDialog,
    edit1: TEdit,
    memo1: TMemo,
    pub btnOpenForm2: TButton,
    form: TForm, // 固定名form, 放最后，前面引用完后，后面move到form。
}

impl TMainForm {
    pub fn new() -> Self {
        let form = Application.CreateForm();
        return Self {
            btn: NewObject!(TButton, form),
            btn2: NewObject!(TButton, form),
            btn3: NewObject!(TButton, form),
            btnOpenDialog: NewObject!(TButton, form),
            btnColorDialog: NewObject!(TButton, form),
            dlgOpen: NewObject!(TOpenDialog, form),
            dlgColor: NewObject!(TColorDialog, form),
            edit1: NewObject!(TEdit, form),
            memo1: NewObject!(TMemo, form),
            btnOpenForm2: NewObject!(TButton, form),
            form: form,
        };
    }

    pub fn init(&self) {
        let sid: usize = self.getSId();

        // TForm
        self.form.SetCaption("你好，Rust！ - Hello Rust!");
        self.form.SetPosition(TPosition::poScreenCenter);
        self.form.SetAllowDropFiles(true);
        self.form.SetOnDropFiles(sid, Self::onDropFile);
        //self.form.SetOnMouseMove(sid, Self::onFormMouseMove);
        self.form.SetColor(clSkyblue);
        self.form.SetKeyPreview(true);
        self.form.SetOnKeyDown(sid, Self::onFormKeyDown);
        self.form.SetOnDestroy(sid, Self::onFormDestroy);

        // form.SetOnClick(onBtnClick);

        // 测试自动drop
        // let ico = TIcon::new();
        //println!("{:?}", ico.ClassName());

        // TOpenDialog
        self.dlgOpen
            .SetFilter("Rust File|*.rs|Other|*.txt;*.c;*.go|All|*.*");
        self.dlgOpen.SetTitle("Open File");

        // TButton
        self.btn.SetParent(self);
        self.btn.SetLeft(10);
        self.btn.SetTop(50);
        self.btn.SetCaption("button1");
        self.btn.SetOnClick(sid, Self::onBtnClick);

        // TButton
        self.btn2.SetParent(self);
        self.btn2.SetLeft(10);
        self.btn2.SetTop(self.btn.Top() + self.btn.Height() + 10);
        self.btn2.SetWidth(120);
        self.btn2.SetCaption("InputCombo");
        self.btn2.SetOnClick(sid, Self::onBtn2Click);

        // TButton
        self.btn3.SetParent(self);
        self.btn3.SetLeft(10);
        self.btn3.SetTop(self.btn2.Top() + self.btn2.Height() + 10);
        self.btn3.SetCaption("MsgBox");
        self.btn3.SetOnClick(sid, Self::onBtn3Click);

        // TButton
        self.btnOpenDialog.SetParent(self);
        self.btnOpenDialog.SetLeft(10);
        self.btnOpenDialog
            .SetTop(self.btn3.Top() + self.btn3.Height() + 10);
        self.btnOpenDialog.SetWidth(120);
        self.btnOpenDialog.SetCaption("Open Dialog");
        self.btnOpenDialog
            .SetOnClick(sid, Self::onBtnOpenDialogClick);

        // TButton
        self.btnColorDialog.SetParent(self);
        self.btnColorDialog.SetLeft(10);
        self.btnColorDialog
            .SetTop(self.btnOpenDialog.Top() + self.btnOpenDialog.Height() + 10);
        self.btnColorDialog.SetWidth(150);
        self.btnColorDialog.SetCaption("Open Color Dialog");
        self.btnColorDialog
            .SetOnClick(sid, Self::onBtnColorDialogClick);

        // TEdit
        self.edit1.SetParent(self);
        self.edit1.SetBounds(
            10,
            self.btnColorDialog.Top() + self.btnColorDialog.Height() + 10,
            300,
            28,
        );
        self.edit1.SetOnChange(sid, Self::onEdit1Change);

        // TMemo
        self.memo1.SetParent(self);
        self.memo1.SetAlign(TAlign::alRight);
        self.memo1.SetWidth(350);
        // 左边相对edit1 + 15距离
        self.memo1
            .AnchorToNeighbour(TAnchorKind::akLeft, 15, &self.edit1);

        // TButton
        self.btnOpenForm2.SetParent(self);
        self.btnOpenForm2.SetLeft(10);
        self.btnOpenForm2.SetCaption("Open Form2");
        self.btnOpenForm2.SetWidth(120);
        self.btnOpenForm2
            .SetTop(self.edit1.Top() + self.edit1.Height() + 10);
        //self.btnOpenForm2.SetOnClick(sid, Self::onOpenForm2Click);
    }

    // fn onOpenForm2Click(&self, _sender: usize) {

    // }

    fn onEdit1Change(&self, _sender: usize) {
        println!("edit1.Change={:}", self.edit1.Text());
    }

    // 按钮1单击事件
    fn onBtnClick(&self, sender: usize) {
        ShowMessage("Hello, Rust! 你好，世界！");
        let btn = TButton::As(sender);
        println!("caption: {:?}", btn.Caption());
    }

    fn onBtn2Click(&self, _sender: usize) {
        //TStrings是抽像类，当参数类型为TStrings要用TStringList传入，非TComponent实现了drop方法，所以Free不是必须的
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

    fn onBtn3Click(&self, _sender: usize) {
        // 集合测试
        // 加法
        let val1 = Include!(0, 1, 2, 3); // result=14
        println!("val1={}", val1);
        // 减法
        let val2 = Exclude!(val1, 2); // result=10
        println!("val2={}", val2);

        println!("2 in set={}", InSet!(val2, 2));
        println!("3 in set={}", InSet!(val2, 3));

        ShowMessage("消息");
        if MessageDlg(
            "消息",
            TMsgDlgType::mtConfirmation,
            Include!(0, TMsgDlgBtn::mbYes, TMsgDlgBtn::mbNo),
            0,
        ) == mrYes
        {
            ShowMessage("你点击了“是")
        }
        // windows consts
        // if Application.MessageBox("消息", "标题", MB_OKCANCEL | MB_ICONINFORMATION) == idOK  {
        //     ShowMessage("你点击了“是")
        // }
    }

    fn onBtnOpenDialogClick(&self, _sender: usize) {
        if self.dlgOpen.Execute() {
            let fileName = self.dlgOpen.FileName();
            println!("fileName={:?}", &fileName);
            self.edit1.SetText(&fileName);
            self.memo1.Lines().LoadFromFile(&fileName);
        }
    }

    fn onBtnColorDialogClick(&self, _sender: usize) {
        if self.dlgColor.Execute() {
            let color = self.dlgColor.Color();
            println!("color={}", color);
            self.form.SetColor(color);
        }
    }

    fn onDropFile(&self, _sender: usize, fileNames: usize, len: isize) {
        //println!("{}, {}, {}", _sender, fileNames, len);
        for i in 0..len {
            let fileName = GetFPStringArrayMember(fileNames, i);
            //println!("{}: {:?}", i, &fileName);
            self.memo1.Lines().Add(&fileName);
        }
    }

    fn onFormMouseMove(&self, _sender: usize, shift: TShiftState, _x: i32, _y: i32) {
        if InSet!(shift, TShiftStateEnum::ssCtrl) {
            println!("ctrl");
        }
        // let form = TForm::As(_sender);
        // let pos = Mouse.CursorPos();
        // println!("x={}, y={}, sx={}, sy={}", _x, _y, pos.x, pos.y);
    }

    fn onFormKeyDown(&self, _sender: usize, key: *mut Char, _shift: TShiftState) {
        unsafe {
            println!("key down={}", *key);
            if *key == vkReturn {
                ShowMessage("down Enter!");
            }
        }
    }

    fn onFormDestroy(&self, _sender: usize) {
        println!("TMainForm destroy.");
    }
}
