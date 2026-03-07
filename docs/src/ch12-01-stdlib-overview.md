# X语言标准库概述

X语言标准库提供了一组核心功能，用于处理常见的编程任务。本文档将详细介绍标准库的主要模块及其功能。

## 标准库版本

标准库当前版本为：

```x
let STDLIB_VERSION: String = "0.1.0"
```

## 核心模块

### Prelude（自动导入）

Prelude模块包含最常用的函数和类型，会自动导入到所有X程序中，无需显式导入。

#### 断言函数

- `assert(condition: Bool, message: String = "断言失败")` - 断言条件为真，否则 panic
- `assert_eq(a, b, message: String = "值不相等")` - 断言两个值相等
- `assert_neq(a, b, message: String = "值相等")` - 断言两个值不相等

#### 转换函数

- `to_string(value): String` - 将值转换为字符串表示
- `type_of(value): String` - 获取值的类型名称

#### 调试辅助

- `dbg(value)` - 打印调试信息（带前缀）并返回值
- `dbg_label(label: String, value)` - 带标签的调试打印并返回值

### Option 模块

Option类型用于表示可能存在或不存在的值，替代了null的使用。

- `Some(value)` - 表示存在值
- `None()` - 表示不存在值
- `is_some(option)` - 检查是否为Some
- `is_none(option)` - 检查是否为None
- `unwrap(option)` - 提取值，如果为None则panic
- `unwrap_or(option, default)` - 提取值，如果为None则返回默认值

### Result 模块

Result类型用于表示可能成功或失败的操作，替代了异常的使用。

- `Ok(value)` - 表示操作成功
- `Err(error)` - 表示操作失败
- `is_ok(result)` - 检查是否为Ok
- `is_err(result)` - 检查是否为Err

## 集合模块

集合模块提供了列表、映射和集合等数据结构的操作。

### 列表 (List) 操作

#### 创建和基本属性

- `list_new<T>(): [T]` - 创建一个新的空列表
- `list_of<T>(item: T): [T]` - 创建一个包含单个元素的列表
- `list_repeat<T>(item: T, count: Int): [T]` - 创建一个包含重复元素的列表
- `list_len<T>(list: [T]): Int` - 获取列表长度
- `list_is_empty<T>(list: [T]): Bool` - 检查列表是否为空

#### 元素访问

- `list_get<T>(list: [T], index: Int): Option<T>` - 获取指定位置的元素
- `list_first<T>(list: [T]): Option<T>` - 获取第一个元素
- `list_last<T>(list: [T]): Option<T>` - 获取最后一个元素

#### 修改操作

- `list_push<T>(list: [T], item: T): [T]` - 在列表末尾添加元素
- `list_pop<T>(list: [T]): (Option<T>, [T])` - 移除并返回列表末尾的元素
- `list_insert<T>(list: [T], index: Int, item: T): [T]` - 在指定位置插入元素
- `list_remove<T>(list: [T], index: Int): (Option<T>, [T])` - 移除指定位置的元素
- `list_clear<T>(list: [T]): [T]` - 清空列表

#### 连接和分割

- `list_append<T>(list1: [T], list2: [T]): [T]` - 连接两个列表
- `list_concat<T>(lists: [[T]]): [T]` - 连接多个列表
- `list_split_at<T>(list: [T], index: Int): ([T], [T])` - 分割列表为两部分

#### 变换操作

- `list_map<T, U>(list: [T], f: (T) -> U): [U]` - 对列表中的每个元素应用函数
- `list_filter<T>(list: [T], predicate: (T) -> Bool): [T]` - 过滤列表，只保留满足谓词的元素
- `list_filter_map<T, U>(list: [T], f: (T) -> Option<U>): [U]` - 过滤并映射列表
- `list_fold<T, U>(list: [T], initial: U, f: (U, T) -> U): U` - 左折叠（从左到右累积）
- `list_fold_right<T, U>(list: [T], initial: U, f: (T, U) -> U): U` - 右折叠（从右到左累积）

#### 搜索操作

- `list_contains<T>(list: [T], item: T): Bool` - 检查列表是否包含指定元素
- `list_find<T>(list: [T], predicate: (T) -> Bool): Option<T>` - 查找第一个满足谓词的元素
- `list_position<T>(list: [T], predicate: (T) -> Bool): Option<Int>` - 查找第一个满足谓词的元素的索引
- `list_all<T>(list: [T], predicate: (T) -> Bool): Bool` - 检查是否所有元素都满足谓词
- `list_any<T>(list: [T], predicate: (T) -> Bool): Bool` - 检查是否有元素满足谓词
- `list_count<T>(list: [T], predicate: (T) -> Bool): Int` - 统计满足谓词的元素数量

#### 排序操作

- `list_reverse<T>(list: [T]): [T]` - 反转列表
- `list_sort_int(list: [Int]): [Int]` - 排序整数列表（升序）
- `list_sort_with<T>(list: [T], compare: (T, T) -> Int): [T]` - 使用比较函数排序

#### 数值操作

- `list_sum(list: [Int]): Int` - 计算整数列表的和
- `list_sum_float(list: [Float]): Float` - 计算浮点数列表的和
- `list_product(list: [Int]): Int` - 计算整数列表的积
- `list_product_float(list: [Float]): Float` - 计算浮点数列表的积
- `list_min_int(list: [Int]): Option<Int>` - 查找整数列表的最小值
- `list_max_int(list: [Int]): Option<Int>` - 查找整数列表的最大值

#### 范围生成

- `list_range(start: Int, end: Int): [Int]` - 创建整数范围 [start, end)
- `list_range_inclusive(start: Int, end: Int): [Int]` - 创建整数范围 [start, end]
- `list_range_step(start: Int, end: Int, step: Int): [Int]` - 创建带步长的范围

#### 切片操作

- `list_slice<T>(list: [T], start: Int, end: Int): [T]` - 获取列表切片 [start, end)
- `list_take<T>(list: [T], n: Int): [T]` - 获取前 n 个元素
- `list_drop<T>(list: [T], n: Int): [T]` - 去掉前 n 个元素

### 映射 (Map) 操作

- `map_new<K, V>(): {K: V}` - 创建一个新的空映射
- `map_is_empty<K, V>(map: {K: V}): Bool` - 检查映射是否为空
- `map_len<K, V>(map: {K: V}): Int` - 获取映射的大小
- `map_get<K, V>(map: {K: V}, key: K): Option<V>` - 获取键对应的值
- `map_insert<K, V>(map: {K: V}, key: K, value: V): {K: V}` - 插入键值对
- `map_remove<K, V>(map: {K: V}, key: K): (Option<V>, {K: V})` - 移除键值对
- `map_contains_key<K, V>(map: {K: V}, key: K): Bool` - 检查映射是否包含键
- `map_keys<K, V>(map: {K: V}): [K]` - 获取所有键
- `map_values<K, V>(map: {K: V}): [V]` - 获取所有值
- `map_entries<K, V>(map: {K: V}): [(K, V)]` - 获取所有键值对
- `map_from_entries<K, V>(entries: [(K, V)]): {K: V}` - 从键值对列表创建映射
- `map_merge<K, V>(map1: {K: V}, map2: {K: V}): {K: V}` - 合并两个映射

### 集合 (Set) 操作

- `set_new<T>(): [T]` - 创建一个新的空集合
- `set_of<T>(items: [T]): [T]` - 创建一个包含元素的集合
- `set_contains<T>(set: [T], item: T): Bool` - 检查集合是否包含元素
- `set_insert<T>(set: [T], item: T): [T]` - 向集合添加元素
- `set_remove<T>(set: [T], item: T): [T]` - 从集合移除元素
- `set_len<T>(set: [T]): Int` - 获取集合大小
- `set_is_empty<T>(set: [T]): Bool` - 检查集合是否为空
- `set_union<T>(set1: [T], set2: [T]): [T]` - 集合的并集
- `set_intersection<T>(set1: [T], set2: [T]): [T]` - 集合的交集
- `set_difference<T>(set1: [T], set2: [T]): [T]` - 集合的差集

## I/O 模块

I/O模块提供了输入输出和文件操作功能。

### 标准输入输出

- `input(): String` - 从标准输入读取一行
- `input_prompt(prompt: String): String` - 从标准输入读取一行，带提示
- `print(...values)` - 打印到标准输出（不换行）
- `println(...values)` - 打印到标准输出（换行）
- `format(template: String, ...args): String` - 格式化字符串

### 文件操作

- `read_file(path: String): Result<String, String>` - 读取文件全部内容为字符串
- `write_file(path: String, content: String): Result<Unit, String>` - 写入字符串到文件
- `append_file(path: String, content: String): Result<Unit, String>` - 追加内容到文件
- `file_exists(path: String): Bool` - 检查文件是否存在
- `delete_file(path: String): Result<Unit, String>` - 删除文件
- `copy_file(from: String, to: String): Result<Unit, String>` - 复制文件
- `move_file(from: String, to: String): Result<Unit, String>` - 移动/重命名文件

### 目录操作

- `create_dir(path: String): Result<Unit, String>` - 创建目录
- `create_dir_all(path: String): Result<Unit, String>` - 创建目录（包括父目录）
- `list_dir(path: String): Result<[String], String>` - 列出目录内容
- `dir_exists(path: String): Bool` - 检查目录是否存在
- `delete_dir(path: String): Result<Unit, String>` - 删除空目录
- `delete_dir_all(path: String): Result<Unit, String>` - 删除目录及其内容
- `current_dir(): Result<String, String>` - 获取当前工作目录
- `set_current_dir(path: String): Result<Unit, String>` - 改变当前工作目录

### 路径操作

- `path_join(parts: [String]): String` - 连接路径
- `path_dirname(path: String): String` - 获取路径的目录部分
- `path_basename(path: String): String` - 获取路径的文件名部分
- `path_extension(path: String): Option<String>` - 获取文件扩展名
- `path_without_extension(path: String): String` - 去除文件扩展名
- `path_is_absolute(path: String): Bool` - 检查路径是否是绝对路径
- `path_is_relative(path: String): Bool` - 检查路径是否是相对路径

### 文件元数据

- `file_size(path: String): Option<Int>` - 获取文件大小（字节）
- `is_file(path: String): Bool` - 检查是否是文件
- `is_dir(path: String): Bool` - 检查是否是目录

### 逐行读取

- `read_lines(path: String): Result<[String], String>` - 读取文件行
- `write_lines(path: String, lines: [String]): Result<Unit, String>` - 写入行到文件
- `append_lines(path: String, lines: [String]): Result<Unit, String>` - 追加行到文件

### 临时文件

- `temp_file(): Result<String, String>` - 创建临时文件
- `temp_dir(): Result<String, String>` - 创建临时目录

### 环境变量

- `env_var(name: String): Option<String>` - 获取环境变量
- `set_env_var(name: String, value: String): Result<Unit, String>` - 设置环境变量
- `env_vars(): Result<{String: String}, String>` - 获取所有环境变量

### 进程操作

- `exit(code: Int): Unit` - 退出程序
- `args(): [String]` - 获取命令行参数
- `program_name(): String` - 获取程序名

### 调试和日志

- `eprint(...values)` - 打印错误信息到标准错误
- `eprintln(...values)` - 打印错误信息到标准错误（带换行）
- `dbg_fmt(template: String, ...args)` - 格式化并打印调试信息

## 网络模块

X语言标准库计划提供网络功能，目前正在开发中。以下是计划中的网络模块功能：

### HTTP 客户端

- `http_get(url: String): Result<String, String>` - 发送HTTP GET请求
- `http_post(url: String, body: String): Result<String, String>` - 发送HTTP POST请求
- `http_request(method: String, url: String, headers: {String: String}, body: String): Result<HttpResponse, String>` - 发送自定义HTTP请求

### TCP 客户端和服务器

- `tcp_connect(host: String, port: Int): Result<TcpStream, String>` - 连接到TCP服务器
- `tcp_listen(host: String, port: Int): Result<TcpListener, String>` - 监听TCP连接
- `tcp_accept(listener: TcpListener): Result<(TcpStream, SocketAddr), String>` - 接受TCP连接

### 套接字操作

- `socket_read(stream: TcpStream, buffer_size: Int): Result<(String, TcpStream), String>` - 从套接字读取数据
- `socket_write(stream: TcpStream, data: String): Result<TcpStream, String>` - 向套接字写入数据
- `socket_close(stream: TcpStream): Result<Unit, String>` - 关闭套接字

### URL 处理

- `url_parse(url: String): Result<Url, String>` - 解析URL
- `url_build(scheme: String, host: String, port: Int, path: String, query: {String: String}): String` - 构建URL

## 时间模块

时间模块提供了时间获取、格式化、睡眠等操作。

### 时间类型

- `Time` - 时间点（自 Unix 纪元以来的秒数和纳秒数）
- `Duration` - 持续时间
- `DateTime` - 日历日期时间

### 时间常量

- `NANOS_PER_SECOND: Int = 1_000_000_000` - 1秒 = 1_000_000_000 纳秒
- `NANOS_PER_MILLISECOND: Int = 1_000_000` - 1毫秒 = 1_000_000 纳秒
- `NANOS_PER_MICROSECOND: Int = 1_000` - 1微秒 = 1_000 纳秒
- `SECONDS_PER_MINUTE: Int = 60` - 1分钟 = 60秒
- `SECONDS_PER_HOUR: Int = 3600` - 1小时 = 3600秒
- `SECONDS_PER_DAY: Int = 86400` - 1天 = 86400秒

### 当前时间

- `timestamp(): Int` - 获取当前时间（自 Unix 纪元以来的秒数）
- `timestamp_millis(): Int` - 获取当前时间（自 Unix 纪元以来的毫秒数）
- `timestamp_micros(): Int` - 获取当前时间（自 Unix 纪元以来的微秒数）
- `timestamp_nanos(): Int` - 获取当前时间（自 Unix 纪元以来的纳秒数）
- `now(): Time` - 获取当前时间点

### 睡眠

- `sleep(seconds: Float)` - 睡眠指定秒数
- `sleep_ms(milliseconds: Int)` - 睡眠指定毫秒数
- `sleep_us(microseconds: Int)` - 睡眠指定微秒数
- `sleep_ns(nanoseconds: Int)` - 睡眠指定纳秒数
- `sleep_duration(duration: Duration)` - 睡眠指定持续时间

### Duration 构造函数

- `duration_seconds(seconds: Int): Duration` - 创建持续时间（秒）
- `duration_millis(milliseconds: Int): Duration` - 创建持续时间（毫秒）
- `duration_micros(microseconds: Int): Duration` - 创建持续时间（微秒）
- `duration_nanos(nanoseconds: Int): Duration` - 创建持续时间（纳秒）
- `duration_minutes(minutes: Int): Duration` - 创建持续时间（分钟）
- `duration_hours(hours: Int): Duration` - 创建持续时间（小时）
- `duration_days(days: Int): Duration` - 创建持续时间（天）

### Duration 操作

- `duration_as_seconds(d: Duration): Float` - 获取持续时间的总秒数
- `duration_as_millis(d: Duration): Int` - 获取持续时间的总毫秒数
- `duration_as_micros(d: Duration): Int` - 获取持续时间的总微秒数
- `duration_as_nanos(d: Duration): Int` - 获取持续时间的总纳秒数
- `duration_add(a: Duration, b: Duration): Duration` - 两个持续时间相加
- `duration_sub(a: Duration, b: Duration): Duration` - 两个持续时间相减
- `duration_compare(a: Duration, b: Duration): Int` - 比较两个持续时间

### Time 操作

- `time_diff(a: Time, b: Time): Duration` - 计算两个时间点的差
- `time_add(t: Time, d: Duration): Time` - 给时间点加上持续时间
- `time_sub(t: Time, d: Duration): Time` - 给时间点减去持续时间
- `time_compare(a: Time, b: Time): Int` - 比较两个时间点

### 日历时间

- `to_local_datetime(seconds: Int): DateTime` - 将时间戳转换为本地日历时间
- `to_utc_datetime(seconds: Int): DateTime` - 将时间戳转换为 UTC 日历时间
- `from_datetime(dt: DateTime): Int` - 将日历时间转换为时间戳
- `local_now(): DateTime` - 获取当前本地时间
- `utc_now(): DateTime` - 获取当前 UTC 时间

### 时间格式化

- `format_datetime(dt: DateTime, format: String): String` - 格式化日期时间为字符串
- `format_iso8601(dt: DateTime): String` - 格式化为 ISO 8601 格式
- `datetime_to_string(dt: DateTime): String` - 简单的日期时间字符串表示

### 工作日和月份

- `weekday_name(weekday: Int): String` - 获取工作日名称
- `weekday_abbr(weekday: Int): String` - 获取工作日缩写
- `month_name(month: Int): String` - 获取月份名称
- `month_abbr(month: Int): String` - 获取月份缩写

### 性能测量

- `time_it<T>(f: () -> T): (T, Float)` - 测量函数执行时间（秒）
- `time_it_print<T>(label: String, f: () -> T): T` - 测量函数执行时间并打印

## 字符串模块

字符串模块提供了丰富的字符串操作功能。

### 字符串基本属性

- `str_len(s: String): Int` - 获取字符串长度（字符数）
- `str_is_empty(s: String): Bool` - 检查字符串是否为空
- `str_byte_len(s: String): Int` - 获取字符串的字节长度

### 字符访问

- `str_chars(s: String): [Char]` - 获取字符串的所有字符
- `str_get(s: String, index: Int): Option<Char>` - 获取指定位置的字符
- `str_first(s: String): Option<Char>` - 获取第一个字符
- `str_last(s: String): Option<Char>` - 获取最后一个字符

### 字符串比较

- `str_compare(a: String, b: String): Int` - 比较两个字符串（字典序）
- `str_eq(a: String, b: String): Bool` - 检查字符串是否相等

### 字符串拼接

- `str_concat(a: String, b: String): String` - 拼接两个字符串
- `str_join(strings: [String], separator: String): String` - 拼接多个字符串
- `str_repeat(s: String, n: Int): String` - 重复字符串 n 次

### 字符串包含检查

- `str_contains(s: String, substr: String): Bool` - 检查字符串是否包含子串
- `str_starts_with(s: String, prefix: String): Bool` - 检查字符串是否以指定前缀开头
- `str_ends_with(s: String, suffix: String): Bool` - 检查字符串是否以指定后缀结尾

### 字符串提取

- `str_substring(s: String, start: Int, end: Int): String` - 提取子字符串
- `str_slice(s: String, start: Int): String` - 提取从 start 到末尾的子串
- `str_take(s: String, n: Int): String` - 获取前 n 个字符
- `str_drop(s: String, n: Int): String` - 去掉前 n 个字符

### 字符串替换

- `str_replace(s: String, from: String, to: String): String` - 替换子字符串
- `str_replace_first(s: String, from: String, to: String): String` - 替换第一个匹配的子字符串

### 字符串大小写转换

- `str_to_lowercase(s: String): String` - 转换为小写
- `str_to_uppercase(s: String): String` - 转换为大写
- `str_capitalize(s: String): String` - 首字母大写

### 字符串修剪

- `str_trim(s: String): String` - 去除首尾空白
- `str_trim_start(s: String): String` - 去除开头空白
- `str_trim_end(s: String): String` - 去除结尾空白
- `str_trim_chars(s: String, chars: String): String` - 去除首尾指定字符
- `str_trim_start_chars(s: String, chars: String): String` - 去除开头指定字符
- `str_trim_end_chars(s: String, chars: String): String` - 去除结尾指定字符

### 字符串填充

- `str_pad_left(s: String, width: Int, pad_char: Char): String` - 左侧填充到指定长度
- `str_pad_right(s: String, width: Int, pad_char: Char): String` - 右侧填充到指定长度
- `str_center(s: String, width: Int, pad_char: Char): String` - 居中填充到指定长度

### 字符串分割

- `str_split(s: String, separator: String): [String]` - 按分隔符分割字符串
- `str_split_whitespace(s: String): [String]` - 按空白分割字符串
- `str_lines(s: String): [String]` - 按行分割字符串

### 字符串解析

- `str_parse_int(s: String): Option<Int>` - 解析为整数
- `str_parse_float(s: String): Option<Float>` - 解析为浮点数
- `str_parse_bool(s: String): Option<Bool>` - 解析为布尔值

### 字符和字符串转换

- `char_to_string(c: Char): String` - 字符转换为字符串
- `char_code(c: Char): Int` - 数字转换为字符
- `char_from_code(code: Int): Option<Char>` - 从字符码创建字符

### 字符分类

- `char_is_alpha(c: Char): Bool` - 检查字符是否是字母
- `char_is_digit(c: Char): Bool` - 检查字符是否是数字
- `char_is_alphanumeric(c: Char): Bool` - 检查字符是否是字母或数字
- `char_is_whitespace(c: Char): Bool` - 检查字符是否是空白
- `char_is_lowercase(c: Char): Bool` - 检查字符是否是小写字母
- `char_is_uppercase(c: Char): Bool` - 检查字符是否是大写字母

### 字符串反转

- `str_reverse(s: String): String` - 反转字符串

### 字符串检查

- `str_is_alpha(s: String): Bool` - 检查字符串是否只包含字母
- `str_is_digit(s: String): Bool` - 检查字符串是否只包含数字
- `str_is_alphanumeric(s: String): Bool` - 检查字符串是否只包含字母或数字
- `str_is_whitespace(s: String): Bool` - 检查字符串是否只包含空白

### 格式化辅助

- `format_int(n: Int, width: Int): String` - 将整数格式化为指定宽度的字符串
- `format_float(n: Float, decimals: Int): String` - 将浮点数格式化为指定小数位数的字符串

## 数学模块

数学模块提供了常用的数学常量和函数。

### 数学常量

- `pi: Float = 3.141592653589793` - 圆周率 π
- `e: Float = 2.718281828459045` - 自然对数的底 e
- `sqrt2: Float = 1.4142135623730951` - 2 的平方根
- `sqrt1_2: Float = 0.7071067811865476` - 1/2 的平方根
- `log2e: Float = 1.4426950408889634` - 以 2 为底 e 的对数
- `log10e: Float = 0.4342944819032518` - 以 10 为底 e 的对数
- `ln2: Float = 0.6931471805599453` - 以 e 为底 2 的对数
- `ln10: Float = 2.302585092994046` - 以 e 为底 10 的对数
- `infinity: Float = 1.0 / 0.0` - 正无穷大
- `neg_infinity: Float = -1.0 / 0.0` - 负无穷大

### 基础函数

- `abs(x: Float): Float` - 绝对值
- `abs_int(x: Int): Int` - 整数绝对值
- `signum(x: Float): Float` - 符号函数：返回 -1, 0, 或 1
- `signum_int(x: Int): Int` - 整数符号函数

### 幂函数和平方根

- `sqrt(x: Float): Float` - 平方根
- `square(x: Float): Float` - 平方
- `square_int(x: Int): Int` - 整数平方
- `pow(x: Float, y: Float): Float` - 幂运算：x^y
- `pow_int(x: Int, y: Int): Int` - 整数幂运算（只支持非负指数）
- `cbrt(x: Float): Float` - 立方根
- `rsqrt(x: Float): Float` - 平方根的倒数

### 指数和对数函数

- `exp(x: Float): Float` - e 的 x 次幂
- `exp2(x: Float): Float` - 2 的 x 次幂
- `ln(x: Float): Float` - 自然对数（以 e 为底）
- `log2(x: Float): Float` - 以 2 为底的对数
- `log10(x: Float): Float` - 以 10 为底的对数
- `log(base: Float, x: Float): Float` - 以任意数为底的对数

### 三角函数

- `sin(x: Float): Float` - 正弦函数（弧度）
- `cos(x: Float): Float` - 余弦函数（弧度）
- `tan(x: Float): Float` - 正切函数（弧度）
- `asin(x: Float): Float` - 反正弦函数（返回弧度）
- `acos(x: Float): Float` - 反余弦函数（返回弧度）
- `atan(x: Float): Float` - 反正切函数（返回弧度）
- `atan2(y: Float, x: Float): Float` - 反正切函数，返回 (x, y) 的角度

### 双曲函数

- `sinh(x: Float): Float` - 双曲正弦
- `cosh(x: Float): Float` - 双曲余弦
- `tanh(x: Float): Float` - 双曲正切

### 取整函数

- `floor(x: Float): Float` - 向下取整
- `ceil(x: Float): Float` - 向上取整
- `round(x: Float): Float` - 四舍五入
- `trunc(x: Float): Float` - 截断小数部分
- `fract(x: Float): Float` - 小数部分

### 极值函数

- `min(a: Float, b: Float): Float` - 两个数中的较小值
- `min_int(a: Int, b: Int): Int` - 两个整数中的较小值
- `max(a: Float, b: Float): Float` - 两个数中的较大值
- `max_int(a: Int, b: Int): Int` - 两个整数中的较大值
- `clamp(x: Float, min_val: Float, max_val: Float): Float` - 限制值在 [min, max] 范围内
- `clamp_int(x: Int, min_val: Int, max_val: Int): Int` - 限制整数在 [min, max] 范围内

### 角度转换

- `radians(degrees: Float): Float` - 将角度转换为弧度
- `degrees(radians: Float): Float` - 将弧度转换为角度

### 距离和插值

- `lerp(a: Float, b: Float, t: Float): Float` - 线性插值：在 a 和 b 之间按 t 插值
- `distance(x1: Float, y1: Float, x2: Float, y2: Float): Float` - 计算两个点之间的欧几里得距离
- `manhattan_distance(x1: Float, y1: Float, x2: Float, y2: Float): Float` - 曼哈顿距离

### 随机数

- `srand(seed: Int)` - 设置随机种子
- `rand(): Float` - 生成 0 到 1 之间的随机浮点数
- `rand_int(min: Int, max: Int): Int` - 生成指定范围内的随机整数 [min, max)
- `rand_float(min: Float, max: Float): Float` - 生成指定范围内的随机浮点数 [min, max)

### 除法和余数

- `div_euclid(a: Int, b: Int): Int` - 欧几里得除法（总是向负无穷方向舍入）
- `rem_euclid(a: Int, b: Int): Int` - 欧几里得余数（总是非负）

### 最大公约数和最小公倍数

- `gcd(a: Int, b: Int): Int` - 最大公约数
- `lcm(a: Int, b: Int): Int` - 最小公倍数

### 因数和质数检查

- `is_even(n: Int): Bool` - 检查是否是偶数
- `is_odd(n: Int): Bool` - 检查是否是奇数
- `is_prime(n: Int): Bool` - 检查是否是质数

### 阶乘和斐波那契

- `factorial(n: Int): Int` - 阶乘
- `fibonacci(n: Int): Int` - 斐波那契数列

## 系统模块

系统模块提供了与操作系统交互的功能。

- `exit(code: Int): Unit` - 退出程序
- `env_var(name: String): Option<String>` - 获取环境变量
- `set_env_var(name: String, value: String): Result<Unit, String>` - 设置环境变量
- `args(): [String]` - 获取命令行参数
- `program_name(): String` - 获取程序名

## 迭代器模块

迭代器模块提供了迭代器相关的功能。

- `iter_range(start: Int, end: Int): [Int]` - 创建整数范围迭代器
- `iter_map<T, U>(iter: [T], f: (T) -> U): [U]` - 映射迭代器
- `iter_filter<T>(iter: [T], predicate: (T) -> Bool): [T]` - 过滤迭代器
- `iter_fold<T, U>(iter: [T], initial: U, f: (U, T) -> U): U` - 折叠迭代器

## 标准库初始化

标准库会在程序启动时自动初始化：

```x
/// 初始化标准库
fun init_stdlib() {
  // 这里可以进行标准库的初始化工作
  // 例如设置随机种子、初始化日志等
  srand(timestamp())
}

// 自动初始化
init_stdlib()
```

## 导入标准库

要使用完整的标准库，只需导入主模块：

```x
import "stdlib"
```

或者导入特定模块：

```x
import "stdlib/collections"
import "stdlib/io"
import "stdlib/time"
```

## 总结

X语言标准库提供了丰富的功能，涵盖了从基本数据结构到高级I/O操作的各个方面。通过这些模块，开发者可以更高效地编写各种类型的应用程序。

标准库的设计理念是简洁、实用和高效，提供了与现代编程语言相匹配的功能集，同时保持了X语言的特色和优势。
