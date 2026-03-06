// 质数判断示例

// 判断一个数是否是质数
function is_prime(n) {
    if (n <= 1) {
        return false
    }
    if (n == 2) {
        return true
    }
    if (n % 2 == 0) {
        return false
    }
    var i = 3
    while (i * i <= n) {
        if (n % i == 0) {
            return false
        }
        i = i + 2
    }
    return true
}

// 测试质数判断
print("Prime number tests:")
var test_numbers = [2, 3, 4, 5, 7, 10, 11, 13, 17, 19, 20, 23, 29, 30]

// 注意：数组语法待完善，先逐个测试
print("Is 2 prime? " + is_prime(2))
print("Is 3 prime? " + is_prime(3))
print("Is 4 prime? " + is_prime(4))
print("Is 5 prime? " + is_prime(5))
print("Is 7 prime? " + is_prime(7))
print("Is 11 prime? " + is_prime(11))

// 找出某个范围内的所有质数
function find_primes_up_to(limit) {
    print("\nPrimes up to " + limit + ":")
    var n = 2
    while (n <= limit) {
        if (is_prime(n)) {
            print(n)
        }
        n = n + 1
    }
}

find_primes_up_to(50)

// 计算第n个质数
function nth_prime(n) {
    var count = 0
    var candidate = 2
    while (true) {
        if (is_prime(candidate)) {
            count = count + 1
            if (count == n) {
                return candidate
            }
        }
        candidate = candidate + 1
    }
}

print("\n10th prime: " + nth_prime(10))
print("20th prime: " + nth_prime(20))
