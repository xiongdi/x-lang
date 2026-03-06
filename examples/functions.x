// 函数示例

// 简单的问候函数
function greet(name) {
    print("Hello, " + name + "!")
}

greet("World")
greet("Alice")
greet("Bob")

// 带返回值的函数
function add(a, b) {
    return a + b
}

val sum = add(5, 7)
print("5 + 7 = " + sum)

// 阶乘函数（递归）
function factorial(n) {
    if (n <= 1) {
        return 1
    }
    return n * factorial(n - 1)
}

print("Factorial of 5: " + factorial(5))
print("Factorial of 10: " + factorial(10))

// 斐波那契函数
function fib(n) {
    if (n <= 1) {
        return n
    }
    return fib(n - 1) + fib(n - 2)
}

print("Fib(10): " + fib(10))

// 最大值函数
function max(a, b) {
    if (a > b) {
        return a
    } else {
        return b
    }
}

print("Max of 10 and 20: " + max(10, 20))
