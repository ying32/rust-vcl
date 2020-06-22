use libc::{c_long, intptr_t, uintptr_t};

pub type TNotifyEvent = fn(uintptr_t);
pub type TDropFilesEvent = fn(uintptr_t, uintptr_t, intptr_t);
