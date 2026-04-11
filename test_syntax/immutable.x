// 测试 let 不可变绑定
let x = 10
// x = 20  // 这应该报错

// 测试 mutable 可变变量
let mutable y = 5
y = 15
println(y)

// 测试 constant 编译期常量
let constant PI = 3.14159
println(PI)