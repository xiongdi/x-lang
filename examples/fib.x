// Fibonacci 数列 - 递归实现
// 用于测试 C23 后端流水线

function fib(n) {
  if n <= 1 {
    return n
  }
  return fib(n - 1) + fib(n - 2)
}

function main() {
  let i = 0
  while i < 10 {
    print("fib(")
    print(i)
    print(") = ")
    print(fib(i))
    print("\n")
    i = i + 1
  }
}
