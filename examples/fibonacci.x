// 斐波那契数列 - X 语言版本
// 用于测试 C23 后端流水线

// 递归版本的 fib(n)
fun fib_recursive(n) {
  if n <= 1 {
    return n
  }
  return fib_recursive(n - 1) + fib_recursive(n - 2)
}

// 迭代版本的 fib(n)
fun fib_iterative(n) {
  if n <= 1 {
    return n
  }
  let a = 0
  let b = 1
  let i = 2
  while i <= n {
    let c = a + b
    a = b
    b = c
    i = i + 1
  }
  return b
}

fun main() {
  print("========================================\n")
  print("  X-Lang - Fibonacci 测试\n")
  print("========================================\n\n")

  print("【测试 1】斐波那契数列（递归版本）\n")
  print("----------------------------------------\n")
  let i = 0
  while i <= 15 {
    print("  fib_recursive(")
    print(i)
    print(") = ")
    print(fib_recursive(i))
    print("\n")
    i = i + 1
  }

  print("\n【测试 2】斐波那契数列（迭代版本）\n")
  print("----------------------------------------\n")
  let j = 0
  while j <= 20 {
    print("  fib_iterative(")
    print(j)
    print(") = ")
    print(fib_iterative(j))
    print("\n")
    j = j + 1
  }

  return 0
}
