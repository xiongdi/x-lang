// @test control flow statements

let num = 10
if num > 0 {
    println("positive")
} else if num == 0 {
    println("zero")
} else {
    println("negative")
}

let num2 = 0
if num2 > 0 {
    println("positive")
} else if num2 == 0 {
    println("zero")
} else {
    println("negative")
}

let num3 = -5
if num3 > 0 {
    println("positive")
} else if num3 == 0 {
    println("zero")
} else {
    println("negative")
}

let mutable count = 0
while count < 3 {
    println("count: " + count)
    count = count + 1
}

let mutable sum = 0
for i in 1..=5 {
    sum = sum + i
}
println("sum: " + sum)
