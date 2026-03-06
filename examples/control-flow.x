// 控制流示例

// if-else 语句
let temperature: integer = 25

if temperature > 30 {
    println("It's hot!")
} else if temperature > 20 {
    println("It's nice!")
} else {
    println("It's cold!")
}

// while 循环
println("Counting from 1 to 5:")
let mutable i: integer = 1
while i <= 5 {
    println(i)
    i += 1
}

// for 循环（待完善，先使用while）
println("Summing 1 to 10:")
let mutable sum: integer = 0
let mutable j: integer = 1
while j <= 10 {
    sum = sum + j
    j += 1
}
println("Sum: {sum}")

// 逻辑运算
let raining: boolean = true
let sunny: boolean = false

if raining and not sunny {
    println("Take an umbrella!")
}

if raining or sunny {
    println("Check the weather!")
}

// 比较运算
let x: integer = 10
let y: integer = 20

if x < y {
    println("x is less than y")
}

if x != y {
    println("x is not equal to y")
}
