use std::mem::{size_of, transmute};

// 根据索引获取参数
unsafe fn get_param_of(index: usize, ptr: usize) -> usize {
    return *((ptr + index * size_of::<usize>()) as *const usize);
}

// 回调函数

pub extern "system" fn do_event_callback(f: usize, args: usize, arg_count: i32) -> usize {
    macro_rules! tt {
        ($x:tt) => {
            usize
        };
    }

    macro_rules! sys_call {
        () => {
            transmute::<usize, fn()>(f)()
        };
        ($($arg:tt),*) => {
            transmute::<usize, fn( $( tt!($arg)),*)>(f)( $(get_param_of($arg, args)),* )
        };
    }
    println!("do_event_callback_proc=({}, {}, {})", f, args, arg_count);
    unsafe {
        match arg_count {
            0 => sys_call!(),
            1 => sys_call!(0),
            2 => sys_call!(0, 1),
            3 => sys_call!(0, 1, 2),
            4 => sys_call!(0, 1, 2, 3),
            5 => sys_call!(0, 1, 2, 3, 4),
            6 => sys_call!(0, 1, 2, 3, 4, 5),
            7 => sys_call!(0, 1, 2, 3, 4, 5, 6),
            8 => sys_call!(0, 1, 2, 3, 4, 5, 6, 7),
            9 => sys_call!(0, 1, 2, 3, 4, 5, 6, 7, 8),
            10 => sys_call!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9),
            11 => sys_call!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10),
            12 => sys_call!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11),
            _ => println!("none"),
        }
    }
    return 0;
}

// 消息回调
pub extern "system" fn do_message_callback(_f: usize, _msg: usize) -> usize {
    return 0;
}

// 线程同步回调
pub extern "system" fn do_thread_sync_callback() -> usize {
    return 0;
}
