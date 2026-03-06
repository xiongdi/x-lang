// 算术运算示例

// 基本算术运算
let a: integer = 10
let b: integer = 3

println("a = {a}")
println("b = {b}")
println("a + b = {a + b}")
println("a - b = {a - b}")
println("a * b = {a * b}")
println("a / b = {a / b}")
println("a % b = {a % b}")

// 复合赋值
let mutable x: integer = 5
println("x = {x}")
x += 3
println("x += 3: {x}")
x *= 2
println("x *= 2: {x}")

// 比较运算
println("\nComparisons:")
println("a == b: {a == b}")
println("a != b: {a != b}")
println("a > b: {a > b}")
println("a < b: {a < b}")
println("a >= b: {a >= b}")
println("a <= b: {a <= b}")

// 逻辑运算
let t: boolean = true
let f: boolean = false

println("\nLogic:")
println("t and f: {t and f}")
println("t or f: {t or f}")
println("not t: {not t}")
println("not f: {not f}")

// 数学函数（待完善内置函数）
function abs(n: integer) -> integer {
    if n < 0 {
        -n
    } else {
        n
    }
}

function max(a: integer, b: integer) -> integer {
    if a > b { a } else { b }
}

function min(a: integer, b: integer) -> integer {
    if a < b { a } else { b }
}

println("\nMath functions:")
println("abs(-42): {abs(-42)}")
println("max(10, 20): {max(10, 20)}")
println("min(10, 20): {min(10, 20)}")
