use std::mem::{size_of, transmute};

// 根据索引获取参数
unsafe fn get_param_of(index: usize, ptr: usize) -> usize {
    return *((ptr + index * size_of::<usize>()) as *const usize);
}

// 回调函数
pub extern "system" fn do_event_callback_proc(f: usize, args: usize, arg_count: i32) -> usize {
    println!("do_event_callback_proc=({}, {}, {})", f, args, arg_count);
    unsafe {
        match arg_count {
            0 => transmute::<usize, fn()>(f)(),
            1 => transmute::<usize, fn(usize)>(f)(get_param_of(0, args)),
            2 => transmute::<usize, fn(usize, usize)>(f)(
                get_param_of(0, args),
                get_param_of(1, args),
            ),
            3 => transmute::<usize, fn(usize, usize, usize)>(f)(
                get_param_of(0, args),
                get_param_of(1, args),
                get_param_of(2, args),
            ),
            4 => transmute::<usize, fn(usize, usize, usize, usize)>(f)(
                get_param_of(0, args),
                get_param_of(1, args),
                get_param_of(2, args),
                get_param_of(3, args),
            ),
            5 => transmute::<usize, fn(usize, usize, usize, usize, usize)>(f)(
                get_param_of(0, args),
                get_param_of(1, args),
                get_param_of(2, args),
                get_param_of(3, args),
                get_param_of(4, args),
            ),
            // 最多12个参数，这里只是演示，所以只实现5个参数的，多的自己实现吧
            6 => transmute::<usize, fn(usize, usize, usize, usize, usize, usize)>(f)(
                get_param_of(0, args),
                get_param_of(1, args),
                get_param_of(2, args),
                get_param_of(3, args),
                get_param_of(4, args),
                get_param_of(5, args),
            ),
            7 => transmute::<usize, fn(usize, usize, usize, usize, usize, usize, usize)>(f)(
                get_param_of(0, args),
                get_param_of(1, args),
                get_param_of(2, args),
                get_param_of(3, args),
                get_param_of(4, args),
                get_param_of(5, args),
                get_param_of(6, args),
            ),
            8 => transmute::<usize, fn(usize, usize, usize, usize, usize, usize, usize, usize)>(f)(
                get_param_of(0, args),
                get_param_of(1, args),
                get_param_of(2, args),
                get_param_of(3, args),
                get_param_of(4, args),
                get_param_of(5, args),
                get_param_of(6, args),
                get_param_of(7, args),
            ),
            9 => transmute::<
                usize,
                fn(usize, usize, usize, usize, usize, usize, usize, usize, usize),
            >(f)(
                get_param_of(0, args),
                get_param_of(1, args),
                get_param_of(2, args),
                get_param_of(3, args),
                get_param_of(4, args),
                get_param_of(5, args),
                get_param_of(6, args),
                get_param_of(7, args),
                get_param_of(8, args),
            ),
            10 => transmute::<
                usize,
                fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize),
            >(f)(
                get_param_of(0, args),
                get_param_of(1, args),
                get_param_of(2, args),
                get_param_of(3, args),
                get_param_of(4, args),
                get_param_of(5, args),
                get_param_of(6, args),
                get_param_of(7, args),
                get_param_of(8, args),
                get_param_of(9, args),
            ),
            11 => transmute::<
                usize,
                fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize),
            >(f)(
                get_param_of(0, args),
                get_param_of(1, args),
                get_param_of(2, args),
                get_param_of(3, args),
                get_param_of(4, args),
                get_param_of(5, args),
                get_param_of(6, args),
                get_param_of(7, args),
                get_param_of(8, args),
                get_param_of(9, args),
                get_param_of(10, args),
            ),
            12 => transmute::<
                usize,
                fn(
                    usize,
                    usize,
                    usize,
                    usize,
                    usize,
                    usize,
                    usize,
                    usize,
                    usize,
                    usize,
                    usize,
                    usize,
                ),
            >(f)(
                get_param_of(0, args),
                get_param_of(1, args),
                get_param_of(2, args),
                get_param_of(3, args),
                get_param_of(4, args),
                get_param_of(5, args),
                get_param_of(6, args),
                get_param_of(7, args),
                get_param_of(8, args),
                get_param_of(9, args),
                get_param_of(10, args),
                get_param_of(11, args),
            ),
            _ => println!("none"),
        }
    }
    return 0;
}
