// 测试 Lambda 表达式
let square = x -> x * x
let add = (a, b) -> a + b
let result1 = square(5)
let result2 = add(3, 4)
println(result1)
println(result2)

// 带类型注解的 Lambda
let multiply = (a: integer, b: integer) -> a * b
println(multiply(6, 7))