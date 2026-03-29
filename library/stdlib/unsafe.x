module std.unsafe

import std::prelude::*;

// === 外部 C 库函数绑定 ===

/// 内存分配
external "c" function malloc(size: usize) -> *()

/// 内存分配并清零
external "c" function calloc(nmemb: usize, size: usize) -> *()

/// 重新分配内存
external "c" function realloc(ptr: *(), size: usize) -> *()

/// 释放内存
external "c" function free(ptr: *()) -> unit

/// 内存复制
external "c" function memcpy(dest: *(), src: *(), n: usize) -> *()

/// 内存移动（处理重叠区域）
external "c" function memmove(dest: *(), src: *(), n: usize) -> *()

/// 内存设置
external "c" function memset(ptr: *(), c: signed 32-bit integer, n: usize) -> *()

/// 内存比较
external "c" function memcmp(a: *(), b: *(), n: usize) -> signed 32-bit integer

/// 获取 NULL 指针
export fn null_ptr() -> *() {
    null
}

/// 检查指针是否为 NULL
export fn is_null(ptr: *()) -> Bool {
    ptr == null
}

/// 分配内存
/// 返回 NULL 如果分配失败
export fn alloc(size: Int) -> *() {
    unsafe {
        malloc(size as usize)
    }
}

/// 分配内存并清零
/// 返回 NULL 如果分配失败
export fn alloc_zeroed(count: Int, size: Int) -> *() {
    unsafe {
        calloc(count as usize, size as usize)
    }
}

/// 重新分配内存
export fn realloc(ptr: *(), new_size: Int) -> *() {
    unsafe {
        realloc(ptr, new_size as usize)
    }
}

/// 释放内存
export fn free(ptr: *()) -> unit {
    unsafe {
        free(ptr)
    }
}

/// 分配单个对象
export fn alloc_one<T>() -> *T {
    unsafe {
        malloc(size_of::<T>() as usize) as *T
    }
}

/// 分配数组
export fn alloc_array<T>(count: Int) -> *T {
    unsafe {
        malloc(count as usize * size_of::<T>() as usize) as *T
    }
}

/// 内存复制
/// 从 src 复制 n 字节到 dest
export fn copy(dest: *(), src: *(), n_bytes: Int) -> *() {
    unsafe {
        memcpy(dest, src, n_bytes as usize)
    }
}

/// 内存移动（处理重叠区域）
export fn move_bytes(dest: *(), src: *(), n_bytes: Int) -> *() {
    unsafe {
        memmove(dest, src, n_bytes as usize)
    }
}

/// 内存设置，将每个字节设置为 c
export fn set_bytes(ptr: *(), c: Int, n_bytes: Int) -> *() {
    unsafe {
        memset(ptr, c as signed 32-bit integer, n_bytes as usize)
    }
}

/// 内存比较，返回负数如果 a < b，零如果相等，正数如果 a > b
export fn compare(a: *(), b: *(), n_bytes: Int) -> Int {
    unsafe {
        memcmp(a, b, n_bytes as usize) as Int
    }
}

/// 判断两块内存是否相等
export fn equal(a: *(), b: *(), n_bytes: Int) -> Bool {
    compare(a, b, n_bytes) == 0
}

/// 读取指针指向的值
export fn read<T>(ptr: *T) -> T {
    unsafe {
        *ptr
    }
}

/// 写入值到指针指向的内存
export fn write<T>(ptr: *mut T, value: T) -> unit {
    unsafe {
        *ptr = value
    }
}

/// 获取指针偏移
export fn byte_offset<T>(ptr: *T, offset: Int) -> *T {
    unsafe {
        ptr + offset
    }
}

/// 获取数组索引处的指针
export fn index<T>(ptr: *T, index: Int) -> *T {
    byte_offset(ptr, index * size_of::<T>())
}

/// 交换两个内存位置的值
export fn swap<T>(a: *mut T, b: *mut T) -> unit {
    unsafe {
        let temp = *a;
        *a = *b;
        *b = temp;
    }
}

/// 将类型转换为指针（对值类型表示其编码后的地址）
/// 这是真正不安全的操作
export fn coerce<T>(value: T) -> *() {
    unsafe {
        value as *()
    }
}

/// 指针类型转换
export fn cast<From, To>(ptr: *From) -> *To {
    unsafe {
        ptr as *To
    }
}

/// 获取类型的大小（字节数）
/// This is a built-in property that the compiler handles
external fn size_of<T>() -> Int;

/// 获取类型的对齐要求（字节数）
external fn align_of<T>() -> Int;

/// 零内存分配器占位符
/// 实际上这由编译器处理
export const null_allocator: *() = null;
