// Minimal prelude with C FFI

/// 外部 C 库函数：puts - 输出字符串并换行
external function puts(message: *character) -> signed 32-bit integer

/// 外部 C 库函数：putchar - 输出单个字符
external function putchar(c: signed 32-bit integer) -> signed 32-bit integer

/// println 函数 - 打印字符串并换行
export function println(message: string) -> unit {
    unsafe {
        puts(message as *character)
    }
}

/// print 函数 - 打印字符串不换行
export function print(message: string) -> unit {
    for c in message {
        unsafe {
            putchar(c as signed 32-bit integer)
        }
    }
}

/// panic 宏 - 终止程序并输出错误信息
export function panic(message: string) -> unit {
    println(message)
}

/// assert 断言 - 如果条件不满足则panic
export function assert(condition: boolean) -> unit {
    if not condition {
        panic("assertion failed")
    }
}
