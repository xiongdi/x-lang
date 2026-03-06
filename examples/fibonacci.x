// 斐波那契数列示例

// 递归版本
function fib_recursive(n: integer) -> integer {
    if n <= 1 { n } else { fib_recursive(n - 1) + fib_recursive(n - 2) }
}

println("Fibonacci (recursive):")
let mutable k: integer = 0
while k <= 10 {
    println("fib({k}) = {fib_recursive(k)}")
    k += 1
}

// 迭代版本（更高效）
function fib_iterative(n: integer) -> integer {
    if n <= 1 { n } else {
        let mutable a: integer = 0
        let mutable b: integer = 1
        let mutable i: integer = 2
        while i <= n {
            let temp: integer = b
            b = a + b
            a = temp
            i += 1
        }
        b
    }
}

println("\nFibonacci (iterative):")
let mutable j: integer = 0
while j <= 20 {
    println("fib({j}) = {fib_iterative(j)}")
    j += 1
}

// 打印斐波那契数列前N项
function print_fib_sequence(count: integer) {
    println("\nFirst {count} Fibonacci numbers:")
    let mutable m: integer = 0
    while m < count {
        println(fib_iterative(m))
        m += 1
    }
}

print_fib_sequence(15)
