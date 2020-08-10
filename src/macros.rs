#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

// 宏定义

#[macro_use]
macro_rules! impl_IObject {
    ($class: ident) => {
        impl IObject for $class {
            fn Instance(&self) -> usize {
                self.0
            }
        }
    };
}

#[macro_use]
macro_rules! impl_IComponent {
    ($class: ident) => {
        impl IComponent for $class {}
    };
}

#[macro_use]
macro_rules! impl_IControl {
    ($class: ident) => {
        impl IControl for $class {}
    };
}

#[macro_use]
macro_rules! impl_IWinControl {
    ($class: ident) => {
        impl IWinControl for $class {}
    };
}

#[macro_use]
macro_rules! impl_IStrings {
    ($class: ident) => {
        impl IStrings for $class {}
    };
}

#[macro_use]
macro_rules! impl_IStream {
    ($class: ident) => {
        impl IStream for $class {}
    };
}

//-------------------------------------------

#[macro_use]
macro_rules! method_Call_1 {
    ($fnName: ident, $($arg:expr),*) => {
        unsafe { $fnName($($arg),* )}
    };
}

#[macro_use]
macro_rules! method_Call_2 {
    ($class: ident, $fnName: ident, $($arg:expr),*) => {
          $class {
              0: unsafe {$fnName($($arg),* )}, 1: false,
          }
    };
}

#[macro_use]
macro_rules! method_Create {
    ($class: ident, $fnName: ident, $($arg:expr),*) => {
          return $class {
              0: unsafe { $fnName($($arg),* ) } , 1: true,
          }
    };
}

#[macro_use]
macro_rules! impl_Class_method {
    ($name: ident) => {
        pub fn Class() -> TClass {
            unsafe { $name() }
        }
    };
}

#[macro_use]
macro_rules! impl_Free_method {
    ($fnName: ident) => {
        pub fn Free(&mut self) {
            unsafe {
                if self.0 > 0 {
                    $fnName(self.0);
                    self.0 = 0;
                }
            }
        }
    };
}

#[macro_use]
macro_rules! impl_Drop_method {
    ($class: ident) => {
        impl Drop for $class {
            fn drop(&mut self) {
                if self.1 {
                    self.Free();
                }
            }
        }
    };
}

#[macro_use]
macro_rules! impl_Object_methods {
    ($class: ident) => {
        // usize to Object
        #[inline]
        pub fn As(inst: usize) -> Self {
            $class { 0: inst, 1: false }
        }

        pub fn from(obj: &dyn IObject) -> Self {
            $class {
                0: obj.Instance(),
                1: false,
            }
        }

        // Nil Object
        #[inline]
        pub fn Nil() -> Self {
            $class { 0: 0, 1: false }
        }

        // Instance is nil
        #[inline]
        pub fn IsNil(&self) -> bool {
            return self.0 == 0;
        }
    };
}

#[macro_use]
macro_rules! to_RustString {
    ($name: expr) => {
        unsafe { CStr::from_ptr($name).to_string_lossy() }
    };
}

#[macro_use]
macro_rules! to_CString {
    ($name: expr) => {
        CString::new($name).unwrap().as_ptr()
    };
}

#[macro_use]
macro_rules! insert_Id {
    ($name1: ident, $name2: expr) => {
        insertMap(unsafe { transmute($name1) }, $name2) //unsafe { transmute($name2) } )
    };
}

//-------------------- exports------------------------------------------

// 集合类型的判断，var2表示位数，下标为0
#[macro_export]
macro_rules! InSet {
    ($var1:expr, $var2:expr) => {
        ($var1 & (1 << $var2 as u8)) != 0
    };
}

// 集合加法，arg...中存储为位的索引，下标为0
#[macro_export]
macro_rules! Include {
    ($var:expr) => {
        $var
    };
    ($var:expr, $($arg:expr),*) => {
        ($var | $( (1 << $arg as u8) )|*)
    };
}

// 集合减法，arg...中存储为位的索引，下标为0
#[macro_export]
macro_rules! Exclude {
    ($var:expr) => {
        $var
    };
    ($var:expr, $($arg:expr),*) => {
        ($var & $( (!(1 << $arg as u8)) )&*)
    };
}

#[macro_export]
macro_rules! ImplISId {
    ($className:ident) => {
        impl ISId for $className {
            fn getSId(&self) -> usize {
                return unsafe { std::mem::transmute(self) };
            }
        }
    };
}

#[macro_export]
macro_rules! ImplIApplication {
    ($className:ident) => {
        impl IApplication for $className {
            fn run(&self) {
                Application.Run();
            }
        }
    };
}

#[macro_export]
macro_rules! ImplForm {
    ($className:ident) => {
        impl IObject for $className {
            fn Instance(&self) -> usize {
                return self.form.Instance();
            }
        }

        impl IComponent for $className {}
        impl IControl for $className {}
        impl IWinControl for $className {}
        impl IForm for $className {}

        impl Drop for $className {
            fn drop(&mut self) {
                println!("drop.");
            }
        }

        ImplISId!($className);
    };
}

// #[macro_export]
// macro_rules! getSId {
//     ($var: expr) => {
//         unsafe { transmute($var) }
//     };
// }

#[macro_export]
macro_rules! NewObject {
    ($class: ident, $obj: ident) => {
        $class::new(&$obj)
    };
}

#[macro_export]
macro_rules! ShowMessageFmt {
    ($($arg:tt)*) => {
        ShowMessage(&format!($($arg)*))
    };
}
