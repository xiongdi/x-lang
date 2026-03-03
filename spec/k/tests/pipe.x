// Pipe operator
fun double(x: Int) = x * 2
fun addOne(x: Int) = x + 1
fun square(x: Int) = x ^ 2

fun main() {
  let result = 5
    |> double
    |> addOne
    |> square
  print(result)
}
