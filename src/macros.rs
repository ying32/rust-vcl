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
macro_rules! impl_As_method {
    ($class: ident) => {
        pub fn As(inst: usize) -> Self {
            $class { 0: inst, 1: false }
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

#[macro_export]
macro_rules! InSet {
    ($var1:expr, $var2:expr) => {
        ($var1 & (1 << $var2 as u8)) != 0
    };
}

#[macro_export]
macro_rules! Include {
    ($var:expr) => {
        $var
    };
    ($var:expr, $($arg:expr),*) => {
        ($var | $( (1 << $arg as u8) )|*)
    };
}

#[macro_export]
macro_rules! Exclude {
    ($var:expr) => {
        $var
    };
    ($var:expr, $($arg:expr),*) => {
        ($var & $( (!(1 << $arg as u8)) )&*)
    };
}
