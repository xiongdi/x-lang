module std

/// 外部 C 库函数：puts - 输出字符串并换行
external "c" function puts(message: *character) -> signed 32-bit integer

/// 外部 C 库函数：putchar - 输出单个字符
external "c" function putchar(c: signed 32-bit integer) -> signed 32-bit integer

/// 外部 C 库函数：fflush - 刷新输出缓冲区
external "c" function fflush(stream: *()) -> signed 32-bit integer

/// 外部 C 库函数：exit - 终止程序
external "c" function exit(code: signed 32-bit integer) -> never

/// 标准输出指针
external "c" variable stdout: *()

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
    unsafe {
        fflush(stdout)
    }
}

/// print_inline 函数 - 打印字符串不换行（别名）
export function print_inline(message: string) -> unit {
    print(message)
}

/// panic 宏 - 终止程序并输出错误信息
export function panic(message: string) -> never {
    println("panic: " ++ message)
    // 调用 exit 退出程序
    unsafe {
        exit(1)
    }
}

/// assert 断言 - 如果条件不满足则panic
export function assert(condition: bool) -> unit {
    when not condition {
        panic("assertion failed")
    }
}

/// assert 断言带自定义消息
export function assert(condition: bool, message: string) -> unit {
    when not condition {
        panic("assertion failed: " ++ message)
    }
}

/// Option 类型 - 表示一个值可能存在或不存在
///
/// Either Some(value) or None
export record Option<T> {
    /// Some(value) - 包含值
    Some(value: T),
    /// None - 不包含值
    None,
}

/// Result 类型 - 表示可能成功或失败
///
/// Either Ok(value) or Err(error)
export record Result<T, E> {
    /// Ok(value) - 成功包含值
    Ok(value: T),
    /// Err(error) - 失败包含错误信息
    Err(error: E),
}

/// 单元类型 - 不包含任何值
export unit Unit

/// 布尔类型预定义
/// 实际上是内置类型，这里只是导出
pub type Bool = bool

/// 整数类型预定义
pub type Int = signed 64-bit integer

/// 浮点数类型预定义
pub type Float = 64-bit float

/// 字符串类型预定义
pub type String = string

/// 字符类型预定义
pub type Char = char
