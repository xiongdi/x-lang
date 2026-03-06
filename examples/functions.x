// 函数示例

// 简单的问候函数
function greet(name: string) {
    println("Hello, {name}!")
}

greet("World")
greet("Alice")
greet("Bob")

// 带返回值的函数
function add(a: integer, b: integer) -> integer = a + b

let sum: integer = add(5, 7)
println("5 + 7 = {sum}")

// 阶乘函数（递归）
function factorial(n: integer) -> integer {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

println("Factorial of 5: {factorial(5)}")
println("Factorial of 10: {factorial(10)}")

// 斐波那契函数
function fib(n: integer) -> integer {
    if n <= 1 { n } else { fib(n - 1) + fib(n - 2) }
}

println("Fib(10): {fib(10)}")

// 最大值函数
function max(a: integer, b: integer) -> integer {
    if a > b { a } else { b }
}

println("Max of 10 and 20: {max(10, 20)}")
