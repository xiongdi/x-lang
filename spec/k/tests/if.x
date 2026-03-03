// If expressions and statements
fun max(a, b) =
  if a > b then a else b

fun main() {
  let x = max(10, 20)
  let y = 0
  if x > 15 {
    y = 1
  } else {
    y = 2
  }
  print(x)
  print(y)
}
