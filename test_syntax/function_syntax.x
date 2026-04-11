// 测试函数定义语法
function greet(name: string) -> string {
    return "Hello, " + name + "!"
}

// 单表达式函数
function square(x: integer) -> integer = x * x

// 数学风格（简洁）
function f(x) -> integer = x * x

println(greet("World"))
println(square(5))
println(f(3))