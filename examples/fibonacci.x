// 斐波那契数列示例

// 递归版本
function fib_recursive(n) {
    if (n <= 1) {
        return n
    }
    return fib_recursive(n - 1) + fib_recursive(n - 2)
}

print("Fibonacci (recursive):")
var k = 0
while (k <= 10) {
    print("fib(" + k + ") = " + fib_recursive(k))
    k = k + 1
}

// 迭代版本（更高效）
function fib_iterative(n) {
    if (n <= 1) {
        return n
    }
    var a = 0
    var b = 1
    var i = 2
    while (i <= n) {
        var temp = b
        b = a + b
        a = temp
        i = i + 1
    }
    return b
}

print("\nFibonacci (iterative):")
var j = 0
while (j <= 20) {
    print("fib(" + j + ") = " + fib_iterative(j))
    j = j + 1
}

// 打印斐波那契数列前N项
function print_fib_sequence(count) {
    print("\nFirst " + count + " Fibonacci numbers:")
    var m = 0
    while (m < count) {
        print(fib_iterative(m))
        m = m + 1
    }
}

print_fib_sequence(15)
