// X语言标准库 - 数学函数
//
// 常用的数学常量和函数
// 底层使用 __rt_* 运行时原语，由各后端内联展开

// ==========================================
// 数学常量
// ==========================================

/// 圆周率 π ≈ 3.141592653589793
let pi: Float = 3.141592653589793

/// 自然对数的底 e ≈ 2.718281828459045
let e: Float = 2.718281828459045

/// 2 的平方根 ≈ 1.4142135623730951
let sqrt2: Float = 1.4142135623730951

/// 1/2 的平方根 ≈ 0.7071067811865476
let sqrt1_2: Float = 0.7071067811865476

/// 以 2 为底 e 的对数 ≈ 1.4426950408889634
let log2e: Float = 1.4426950408889634

/// 以 10 为底 e 的对数 ≈ 0.4342944819032518
let log10e: Float = 0.4342944819032518

/// 以 e 为底 2 的对数 ≈ 0.6931471805599453
let ln2: Float = 0.6931471805599453

/// 以 e 为底 10 的对数 ≈ 2.302585092994046
let ln10: Float = 2.302585092994046

/// 正无穷大
let infinity: Float = 1.0 / 0.0

/// 负无穷大
let neg_infinity: Float = -1.0 / 0.0

// ==========================================
// 基础函数
// ==========================================

/// 绝对值（浮点数）
function abs(x: Float): Float {
  __rt_abs_float(x)
}

/// 绝对值（整数）
function abs_int(x: Int): Int {
  __rt_abs_int(x)
}

/// 符号函数：返回 -1, 0, 或 1（浮点数）
function signum(x: Float): Float {
  if x > 0.0 { 1.0 }
  else if x < 0.0 { -1.0 }
  else { 0.0 }
}

/// 符号函数（整数）
function signum_int(x: Int): Int {
  if x > 0 { 1 }
  else if x < 0 { -1 }
  else { 0 }
}

// ==========================================
// 幂函数和平方根
// ==========================================

/// 平方根
function sqrt(x: Float): Float {
  __rt_sqrt(x)
}

/// 平方
function square(x: Float): Float {
  x * x
}

/// 整数平方
function square_int(x: Int): Int {
  x * x
}

/// 幂运算：x^y
function pow(x: Float, y: Float): Float {
  __rt_pow(x, y)
}

/// 整数幂运算（只支持非负指数）
function pow_int(x: Int, y: Int): Int {
  if y < 0 {
    panic("pow_int: 指数不能为负")
  }
  let mut result = 1
  let mut base = x
  let mut exp = y
  while exp > 0 {
    if exp % 2 == 1 {
      result = result * base
    }
    base = base * base
    exp = exp / 2
  }
  result
}

/// 立方根
function cbrt(x: Float): Float {
  pow(x, 1.0 / 3.0)
}

/// 平方根的倒数
function rsqrt(x: Float): Float {
  1.0 / sqrt(x)
}

// ==========================================
// 指数和对数函数
// ==========================================

/// e 的 x 次幂
function exp(x: Float): Float {
  __rt_exp(x)
}

/// 2 的 x 次幂
function exp2(x: Float): Float {
  pow(2.0, x)
}

/// 自然对数（以 e 为底）
function ln(x: Float): Float {
  __rt_log(x)
}

/// 以 2 为底的对数
function log2(x: Float): Float {
  __rt_log2(x)
}

/// 以 10 为底的对数
function log10(x: Float): Float {
  __rt_log10(x)
}

/// 以任意数为底的对数
function log(base: Float, x: Float): Float {
  ln(x) / ln(base)
}

// ==========================================
// 三角函数
// ==========================================

/// 正弦函数（弧度）
function sin(x: Float): Float {
  __rt_sin(x)
}

/// 余弦函数（弧度）
function cos(x: Float): Float {
  __rt_cos(x)
}

/// 正切函数（弧度）
function tan(x: Float): Float {
  __rt_tan(x)
}

/// 反正弦函数（返回弧度）
function asin(x: Float): Float {
  __rt_asin(x)
}

/// 反余弦函数（返回弧度）
function acos(x: Float): Float {
  __rt_acos(x)
}

/// 反正切函数（返回弧度）
function atan(x: Float): Float {
  __rt_atan(x)
}

/// 反正切函数，返回 (x, y) 的角度
function atan2(y: Float, x: Float): Float {
  __rt_atan2(y, x)
}

// ==========================================
// 双曲函数
// ==========================================

/// 双曲正弦
function sinh(x: Float): Float {
  (exp(x) - exp(-x)) / 2.0
}

/// 双曲余弦
function cosh(x: Float): Float {
  (exp(x) + exp(-x)) / 2.0
}

/// 双曲正切
function tanh(x: Float): Float {
  sinh(x) / cosh(x)
}

// ==========================================
// 取整函数
// ==========================================

/// 向下取整
function floor(x: Float): Float {
  __rt_floor(x)
}

/// 向上取整
function ceil(x: Float): Float {
  __rt_ceil(x)
}

/// 四舍五入
function round(x: Float): Float {
  __rt_round(x)
}

/// 截断小数部分
function trunc(x: Float): Float {
  __rt_trunc(x)
}

/// 小数部分
function fract(x: Float): Float {
  x - trunc(x)
}

// ==========================================
// 极值函数
// ==========================================

/// 两个浮点数中的较小值
function min(a: Float, b: Float): Float {
  __rt_min_float(a, b)
}

/// 两个整数中的较小值
function min_int(a: Int, b: Int): Int {
  __rt_min_int(a, b)
}

/// 两个浮点数中的较大值
function max(a: Float, b: Float): Float {
  __rt_max_float(a, b)
}

/// 两个整数中的较大值
function max_int(a: Int, b: Int): Int {
  __rt_max_int(a, b)
}

/// 限制值在 [min, max] 范围内（浮点数）
function clamp(x: Float, min_val: Float, max_val: Float): Float {
  min(max(x, min_val), max_val)
}

/// 限制整数在 [min, max] 范围内
function clamp_int(x: Int, min_val: Int, max_val: Int): Int {
  min_int(max_int(x, min_val), max_val)
}

// ==========================================
// 角度转换
// ==========================================

/// 将角度转换为弧度
function radians(degrees: Float): Float {
  degrees * pi / 180.0
}

/// 将弧度转换为角度
function degrees(radians: Float): Float {
  radians * 180.0 / pi
}

// ==========================================
// 距离和插值
// ==========================================

/// 线性插值：在 a 和 b 之间按 t 插值
function lerp(a: Float, b: Float, t: Float): Float {
  a + t * (b - a)
}

/// 计算两个点之间的欧几里得距离
function distance(x1: Float, y1: Float, x2: Float, y2: Float): Float {
  sqrt(square(x2 - x1) + square(y2 - y1))
}

/// 曼哈顿距离
function manhattan_distance(x1: Float, y1: Float, x2: Float, y2: Float): Float {
  abs(x2 - x1) + abs(y2 - y1)
}

// ==========================================
// 随机数（伪随机，简单实现）
// ==========================================

let mut rng_seed: Int = 12345

/// 设置随机种子
function srand(seed: Int) {
  rng_seed = seed
}

/// 生成 0 到 1 之间的随机浮点数
function rand(): Float {
  // 简单的 LCG 随机数生成器
  rng_seed = (rng_seed * 1103515245 + 12345) % 2147483648
  abs_int(rng_seed) as Float / 2147483648.0
}

/// 生成指定范围内的随机整数 [min, max)
function rand_int(min: Int, max: Int): Int {
  min + (rand() * (max - min) as Float) as Int
}

/// 生成指定范围内的随机浮点数 [min, max)
function rand_float(min: Float, max: Float): Float {
  min + rand() * (max - min)
}

// ==========================================
// 除法和余数
// ==========================================

/// 欧几里得除法（总是向负无穷方向舍入）
function div_euclid(a: Int, b: Int): Int {
  let q = a / b
  if (a % b < 0 && b > 0) { q - 1 }
  else if (a % b > 0 && b < 0) { q - 1 }
  else { q }
}

/// 欧几里得余数（总是非负）
function rem_euclid(a: Int, b: Int): Int {
  a - div_euclid(a, b) * b
}

// ==========================================
// 最大公约数和最小公倍数
// ==========================================

/// 最大公约数
function gcd(a: Int, b: Int): Int {
  let mut x = abs_int(a)
  let mut y = abs_int(b)
  while y != 0 {
    let temp = y
    y = x % y
    x = temp
  }
  x
}

/// 最小公倍数
function lcm(a: Int, b: Int): Int {
  if a == 0 || b == 0 { 0 }
  else { abs_int(a * b) / gcd(a, b) }
}

// ==========================================
// 因数和质数检查
// ==========================================

/// 检查是否是偶数
function is_even(n: Int): Bool {
  n % 2 == 0
}

/// 检查是否是奇数
function is_odd(n: Int): Bool {
  n % 2 != 0
}

/// 检查是否是质数
function is_prime(n: Int): Bool {
  if n <= 1 { false }
  else if n == 2 { true }
  else if is_even(n) { false }
  else {
    let mut i = 3
    while i * i <= n {
      if n % i == 0 {
        return false
      }
      i = i + 2
    }
    true
  }
}

// ==========================================
// 阶乘
// ==========================================

/// 阶乘
function factorial(n: Int): Int {
  if n < 0 {
    panic("factorial: 参数不能为负")
  }
  let mut result = 1
  let mut i = 2
  while i <= n {
    result = result * i
    i = i + 1
  }
  result
}

/// 斐波那契数列
function fibonacci(n: Int): Int {
  if n < 0 {
    panic("fibonacci: 参数不能为负")
  }
  if n == 0 { 0 }
  else if n == 1 { 1 }
  else {
    let mut a = 0
    let mut b = 1
    let mut i = 2
    while i <= n {
      let temp = b
      b = a + b
      a = temp
      i = i + 1
    }
    b
  }
}
