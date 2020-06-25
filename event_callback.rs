use libc::{c_long, intptr_t, uintptr_t};
use std::mem::{size_of, transmute};

// 根据索引获取参数
unsafe fn get_param_of(index: uintptr_t, ptr: uintptr_t) -> uintptr_t {
    return *((ptr + index * size_of::<uintptr_t>()) as *const uintptr_t);
}

// 回调函数
pub extern "system" fn do_event_callback_proc(
    f: uintptr_t,
    args: uintptr_t,
    arg_count: c_long,
) -> uintptr_t {
    println!("do_event_callback_proc=({}, {}, {})", f, args, arg_count);
    unsafe {
        match arg_count {
            0 => transmute::<uintptr_t, fn()>(f)(),
            1 => transmute::<uintptr_t, fn(uintptr_t)>(f)(get_param_of(0, args)),
            2 => transmute::<uintptr_t, fn(uintptr_t, uintptr_t)>(f)(
                get_param_of(0, args),
                get_param_of(1, args),
            ),
            3 => transmute::<uintptr_t, fn(uintptr_t, uintptr_t, uintptr_t)>(f)(
                get_param_of(0, args),
                get_param_of(1, args),
                get_param_of(2, args),
            ),
            4 => transmute::<uintptr_t, fn(uintptr_t, uintptr_t, uintptr_t, uintptr_t)>(f)(
                get_param_of(0, args),
                get_param_of(1, args),
                get_param_of(2, args),
                get_param_of(3, args),
            ),
            5 => {
                transmute::<uintptr_t, fn(uintptr_t, uintptr_t, uintptr_t, uintptr_t, uintptr_t)>(f)(
                    get_param_of(0, args),
                    get_param_of(1, args),
                    get_param_of(2, args),
                    get_param_of(3, args),
                    get_param_of(4, args),
                )
            }
            // 最多12个参数，这里只是演示，所以只实现5个参数的，多的自己实现吧
            6 => transmute::<
                uintptr_t,
                fn(uintptr_t, uintptr_t, uintptr_t, uintptr_t, uintptr_t, uintptr_t),
            >(f)(
                get_param_of(0, args),
                get_param_of(1, args),
                get_param_of(2, args),
                get_param_of(3, args),
                get_param_of(4, args),
                get_param_of(5, args),
            ),
            7 => transmute::<
                uintptr_t,
                fn(uintptr_t, uintptr_t, uintptr_t, uintptr_t, uintptr_t, uintptr_t, uintptr_t),
            >(f)(
                get_param_of(0, args),
                get_param_of(1, args),
                get_param_of(2, args),
                get_param_of(3, args),
                get_param_of(4, args),
                get_param_of(5, args),
                get_param_of(6, args),
            ),
            8 => transmute::<
                uintptr_t,
                fn(
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
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
            ),
            9 => transmute::<
                uintptr_t,
                fn(
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
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
            ),
            10 => transmute::<
                uintptr_t,
                fn(
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
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
            ),
            11 => transmute::<
                uintptr_t,
                fn(
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
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
            ),
            12 => transmute::<
                uintptr_t,
                fn(
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
                    uintptr_t,
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
