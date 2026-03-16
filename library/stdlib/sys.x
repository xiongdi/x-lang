// X语言标准库 - 系统功能
//
// 系统相关操作，如环境变量、命令行参数、进程操作等
// 底层使用 __rt_* 运行时原语，由各后端内联展开

// ==========================================
// 环境变量
// ==========================================

/// 获取环境变量
function get_env(name: String): Option<String> {
  __rt_get_env(name)
}

/// 设置环境变量
function set_env(name: String, value: String): Bool {
  __rt_set_env(name, value)
}

/// 删除环境变量
function unset_env(name: String): Bool {
  // 设置为空字符串作为删除的近似实现
  __rt_set_env(name, "")
}

/// 获取所有环境变量
function env_vars(): List<(String, String)> {
  // 简化实现，需要更复杂的底层支持
  []
}

// ==========================================
// 命令行参数
// ==========================================

/// 获取命令行参数
function args(): List<String> {
  __rt_args()
}

/// 获取命令行参数数量
function arg_count(): Int {
  let args_list = args()
  len(args_list)
}

/// 获取指定索引的命令行参数
function arg(index: Int): Option<String> {
  let args_list = args()
  if index >= 0 && index < len(args_list) {
    Some(args_list[index])
  } else {
    None
  }
}

// ==========================================
// 进程操作
// ==========================================

/// 获取当前进程ID
function getpid(): Int {
  __rt_getpid()
}

/// 获取父进程ID（部分平台支持）
function getppid(): Int {
  // 需要平台特定的实现
  -1
}

/// 终止当前进程
function exit(code: Int) {
  __rt_exit(code)
}

/// 执行系统命令（简化实现）
function system(command: String): Int {
  // 需要平台特定的实现
  -1
}

/// 执行命令并获取输出（简化实现）
function command_output(command: String): Result<String, String> {
  Err("Command execution not implemented")
}

// ==========================================
// 系统信息
// ==========================================

/// 获取操作系统类型
function os_type(): String {
  match get_env("OS") is
    Some { value: os } -> if os == "Windows_NT" { "Windows" } else { os }
    None -> match get_env("OSTYPE") is
      Some { value: t } -> t
      None -> "Unknown"
}

/// 获取操作系统版本
function os_version(): String {
  // 需要平台特定的实现
  "Unknown"
}

/// 获取主机名
function hostname(): String {
  // 需要平台特定的实现
  match get_env("HOSTNAME") is
    Some { value: h } -> h
    None -> match get_env("COMPUTERNAME") is
      Some { value: c } -> c
      None -> "Unknown"
}

/// 获取系统架构
function arch(): String {
  // 需要平台特定的实现
  match get_env("PROCESSOR_ARCHITECTURE") is
    Some { value: a } -> a
    None -> "Unknown"
}

/// 获取可用内存（字节）
function free_memory(): Int {
  // 需要平台特定的实现
  0
}

/// 获取总内存（字节）
function total_memory(): Int {
  // 需要平台特定的实现
  0
}

/// 获取CPU核心数
function cpu_count(): Int {
  // 需要平台特定的实现
  match get_env("NUMBER_OF_PROCESSORS") is
    Some { value: n } -> match str_to_int(n) is
      Some { value: count } -> count
      None -> 1
    None -> 1
}

// ==========================================
// 路径操作
// ==========================================

/// 获取当前工作目录
function current_dir(): Result<String, String> {
  __rt_cwd()
}

/// 改变工作目录
function chdir(path: String): Result<Unit, String> {
  __rt_chdir(path)
}

/// 拼接路径
function path_join(paths: List<String>): String {
  if len(paths) == 0 {
    ""
  } else if len(paths) == 1 {
    paths[0]
  } else {
    let sep = if os_type() == "Windows" { "\\" } else { "/" }
    let mut result = paths[0]
    for i in 1..len(paths) {
      let path = paths[i]
      if result != "" && !str_ends_with(result, sep) {
        result = result + sep
      }
      if str_starts_with(path, sep) {
        result = result + str_substring(path, 1)
      } else {
        result = result + path
      }
    }
    result
  }
}

/// 获取路径的目录部分
function path_dirname(path: String): String {
  // 简化实现
  let parts = str_split(path, "/")
  if len(parts) <= 1 {
    return "."
  }
  let dir_parts = list_slice(parts, 0, len(parts) - 1)
  str_join(dir_parts, "/")
}

/// 获取路径的文件名部分
function path_basename(path: String): String {
  let parts = str_split(path, "/")
  if list_is_empty(parts) {
    return ""
  }
  list_last(parts)
}

/// 获取路径的扩展名
function path_extension(path: String): String {
  let basename = path_basename(path)
  let parts = str_split(basename, ".")
  if len(parts) > 1 {
    list_last(parts)
  } else {
    ""
  }
}

/// 检查路径是否存在
function path_exists(path: String): Bool {
  __rt_file_exists(path) || __rt_dir_exists(path)
}

/// 检查路径是否为文件
function is_file(path: String): Bool {
  __rt_file_exists(path) && !__rt_dir_exists(path)
}

/// 检查路径是否为目录
function is_dir(path: String): Bool {
  __rt_dir_exists(path)
}

// ==========================================
// 临时文件
// ==========================================

/// 创建临时文件
function temp_file(): Result<String, String> {
  let ts = __rt_timestamp_ms()
  let name = if os_type() == "Windows" {
    match get_env("TEMP") is
      Some { value: tmp } -> tmp + "\\xlang_temp_" + to_string(ts)
      None -> ".\\xlang_temp_" + to_string(ts)
  } else {
    "/tmp/xlang_temp_" + to_string(ts)
  }
  __rt_file_write(name, "")
  Ok(name)
}

/// 创建临时目录
function temp_dir(): Result<String, String> {
  let ts = __rt_timestamp_ms()
  let name = if os_type() == "Windows" {
    match get_env("TEMP") is
      Some { value: tmp } -> tmp + "\\xlang_temp_dir_" + to_string(ts)
      None -> ".\\xlang_temp_dir_" + to_string(ts)
  } else {
    "/tmp/xlang_temp_dir_" + to_string(ts)
  }
  __rt_dir_create(name)
  Ok(name)
}

/// 获取系统临时目录
function get_temp_dir(): String {
  if os_type() == "Windows" {
    match get_env("TEMP") is
      Some { value: tmp } -> tmp
      None -> "."
  } else {
    "/tmp"
  }
}

// ==========================================
// 信号处理
// ==========================================

/// 信号类型
type Signal = {
  name: String,
  number: Int,
}

/// 信号定义
let SIGINT: Signal = { name: "SIGINT", number: 2 }     // 中断信号
let SIGTERM: Signal = { name: "SIGTERM", number: 15 }   // 终止信号
let SIGKILL: Signal = { name: "SIGKILL", number: 9 }   // 强制终止信号
let SIGSEGV: Signal = { name: "SIGSEGV", number: 11 }  // 段错误

/// 注册信号处理器（简化实现）
function signal(signum: Int, handler: () -> ()): Bool {
  // 需要平台特定的实现
  false
}

/// 发送信号到进程
function kill(pid: Int, signum: Int): Bool {
  // 需要平台特定的实现
  false
}

// ==========================================
// 系统调用
// ==========================================

/// 系统调用（底层）
function syscall(number: Int, args: List<Int>): Int {
  // 需要平台特定的实现
  -1
}

// ==========================================
// 时间相关系统函数
// ==========================================

/// 获取当前时间戳（毫秒）
function timestamp_ms(): Int {
  __rt_timestamp_ms()
}

/// 获取当前时间戳（纳秒）
function timestamp_ns(): Int {
  __rt_timestamp_ns()
}

/// 获取系统启动时间（秒）
function uptime(): Float {
  // 简化实现
  __rt_timestamp_ms() as Float / 1000.0
}

/// 休眠指定毫秒数
function sleep(ms: Int) {
  __rt_sleep(ms)
}

// ==========================================
// 随机数
// ==========================================

let mut sys_rng_seed: Int = 12345

/// 设置随机数种子
function srand(seed: Int) {
  sys_rng_seed = seed
}

/// 生成随机整数（0 到 max-1）
function random(max: Int): Int {
  sys_rng_seed = (sys_rng_seed * 1103515245 + 12345) % 2147483648
  let result = sys_rng_seed % max
  if result < 0 { -result } else { result }
}

/// 生成随机浮点数（0.0 到 1.0）
function random_float(): Float {
  sys_rng_seed = (sys_rng_seed * 1103515245 + 12345) % 2147483648
  let abs_val = if sys_rng_seed < 0 { -sys_rng_seed } else { sys_rng_seed }
  abs_val as Float / 2147483648.0
}

// ==========================================
// 其他系统函数
// ==========================================

/// 获取用户ID
function getuid(): Int {
  // 需要平台特定的实现
  -1
}

/// 获取组ID
function getgid(): Int {
  // 需要平台特定的实现
  -1
}

/// 获取用户名
function get_username(): String {
  match get_env("USER") is
    Some { value: u } -> u
    None -> match get_env("USERNAME") is
      Some { value: u } -> u
      None -> "Unknown"
}

/// 获取组名
function get_groupname(): String {
  // 需要平台特定的实现
  "Unknown"
}
