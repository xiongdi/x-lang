// 算术运算示例

// 基本算术运算
val a = 10
val b = 3

print("a = " + a)
print("b = " + b)
print("a + b = " + (a + b))
print("a - b = " + (a - b))
print("a * b = " + (a * b))
print("a / b = " + (a / b))
print("a % b = " + (a % b))

// 复合赋值
var x = 5
print("x = " + x)
x = x + 3
print("x = x + 3: " + x)
x = x * 2
print("x = x * 2: " + x)

// 比较运算
print("\nComparisons:")
print("a == b: " + (a == b))
print("a != b: " + (a != b))
print("a > b: " + (a > b))
print("a < b: " + (a < b))
print("a >= b: " + (a >= b))
print("a <= b: " + (a <= b))

// 逻辑运算
val t = true
val f = false

print("\nLogic:")
print("t && f: " + (t && f))
print("t || f: " + (t || f))
print("!t: " + (!t))
print("!f: " + (!f))

// 数学函数（待完善内置函数）
function abs(n) {
    if (n < 0) {
        return -n
    }
    return n
}

function max(a, b) {
    if (a > b) {
        return a
    }
    return b
}

function min(a, b) {
    if (a < b) {
        return a
    }
    return b
}

print("\nMath functions:")
print("abs(-42): " + abs(-42))
print("max(10, 20): " + max(10, 20))
print("min(10, 20): " + min(10, 20))
