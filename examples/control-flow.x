// 控制流示例

// if-else 语句
val temperature = 25

if (temperature > 30) {
    print("It's hot!")
} else if (temperature > 20) {
    print("It's nice!")
} else {
    print("It's cold!")
}

// while 循环
print("Counting from 1 to 5:")
var i = 1
while (i <= 5) {
    print(i)
    i = i + 1
}

// for 循环（待完善，先使用while）
print("Summing 1 to 10:")
var sum = 0
var j = 1
while (j <= 10) {
    sum = sum + j
    j = j + 1
}
print("Sum: " + sum)

// 逻辑运算
val raining = true
val sunny = false

if (raining && !sunny) {
    print("Take an umbrella!")
}

if (raining || sunny) {
    print("Check the weather!")
}

// 比较运算
val x = 10
val y = 20

if (x < y) {
    print("x is less than y")
}

if (x != y) {
    print("x is not equal to y")
}
