// Loops
fun sumList(numbers: [Int]): Int {
  let mut sum = 0
  for n in numbers {
    sum = sum + n
  }
  sum
}

fun factorialLoop(n: Int): Int {
  let mut result = 1
  let mut i = 1
  while i <= n {
    result = result * i
    i = i + 1
  }
  result
}

fun main() {
  let sum = sumList([1, 2, 3, 4, 5])
  let fact = factorialLoop(5)
  print(sum)
  print(fact)
}
