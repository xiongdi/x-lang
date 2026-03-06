// 质数判断示例

// 判断一个数是否是质数
function is_prime(n: integer) -> boolean {
    if n <= 1 { false }
    else if n == 2 { true }
    else if n % 2 == 0 { false }
    else {
        let mutable i: integer = 3
        while i * i <= n {
            if n % i == 0 {
                return false
            }
            i += 2
        }
        true
    }
}

// 测试质数判断
println("Prime number tests:")
let test_numbers: [integer] = [2, 3, 4, 5, 7, 10, 11, 13, 17, 19, 20, 23, 29, 30]
for n in test_numbers {
    println("Is {n} prime? {is_prime(n)}")
}

// 找出某个范围内的所有质数
function find_primes_up_to(limit: integer) {
    println("\nPrimes up to {limit}:")
    for n in 2..=limit {
        if is_prime(n) {
            println(n)
        }
    }
}

find_primes_up_to(50)

// 计算第n个质数
function nth_prime(n: integer) -> integer {
    let mutable count: integer = 0
    let mutable candidate: integer = 2
    while true {
        if is_prime(candidate) {
            count += 1
            if count == n {
                return candidate
            }
        }
        candidate += 1
    }
}

println("\n10th prime: {nth_prime(10)}")
println("20th prime: {nth_prime(20)}")
