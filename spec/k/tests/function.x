// Function definition and application
function add(a, b) = a + b

function factorial(n) =
  if n <= 1 then
    1
  else
    n * factorial(n - 1)

function main() {
  let sum = add(2, 3)
  let fact = factorial(5)
  print(sum)
  print(fact)
}
