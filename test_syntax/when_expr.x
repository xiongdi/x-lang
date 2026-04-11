// 测试 when 表达式（模式匹配）
let x = 5
let desc = when x is {
    1 => "one"
    2 => "two"
    n if n > 3 => "big"
    _ => "other"
}
println(desc)

// 测试通配符模式
let y = when 10 is {
    100 => "hundred"
    _ => "other"
}
println(y)