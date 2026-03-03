// Loops
function sumList(numbers: [integer 32]): integer 32 {
  let mutable sum = 0
  for n in numbers {
    sum = sum + n
  }
  sum
}

function factorialLoop(n: integer 32): integer 32 {
  let mutable result = 1
  let mutable i = 1
  while i <= n {
    result = result * i
    i = i + 1
  }
  result
}

function main() {
  let sum = sumList([1, 2, 3, 4, 5])
  let fact = factorialLoop(5)
  print(sum)
  print(fact)
}
