// Pipe operator
function double(x: integer 32) = x * 2
function addOne(x: integer 32) = x + 1
function square(x: integer 32) = x ^ 2

function main() {
  let result = 5
    |> double
    |> addOne
    |> square
  print(result)
}
