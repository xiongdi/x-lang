function double(x: Int) -> Int = x * 2
function square(x: Int) -> Int = x * x

function main() {
  let result = 5 |> double |> square
  println(result)
}
