// 测试枚举定义
enum Color {
    Red
    Green
    Blue
    RGB(integer, integer, integer)
}

let c1 = Color.Red
let c2 = Color.RGB(255, 128, 0)
println(c1)
println(c2)

// 测试模式匹配
let desc = when c2 is {
    Color.Red => "red"
    Color.Green => "green"
    Color.Blue => "blue"
    Color.RGB(r, g, b) => "rgb(" + r + "," + g + "," + b + ")"
}
println(desc)