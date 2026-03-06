// 变量和数据类型示例

// 不可变绑定 (let) - 默认不可变
let message: string = "Hello, X Language!"
println(message)

// 可变变量 (let mutable)
let mutable count: integer = 0
count += 1
println("Count: {count}")

// 基本数据类型
let integer: integer = 42
let floating: float = 3.14159
let boolean: boolean = true
let character: character = 'X'

println("Integer: {integer}")
println("Float: {floating}")
println("Boolean: {boolean}")
println("Char: {character}")

// 多变量声明（当前语法不支持 `let a = 10, b = 20` 这种写法）
let a: integer = 10
let b: integer = 20
println("a + b = {a + b}")
