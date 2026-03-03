// Function definition and application
add(a, b) = a + b

fun factorial(n) =
  if n <= 1 then
    1
  else
    n * factorial(n - 1)

fun main() {
  let sum = add(2, 3)
  let fact = factorial(5)
  print(sum)
  print(fact)
}
