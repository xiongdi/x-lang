// Result type
type DivError = DivisionByZero

fun divide(a: Int, b: Int): Result<Int, DivError> =
  if b == 0 then
    Err(DivisionByZero)
  else
    Ok(a / b)

fun main() {
  let ok = divide(10, 2)
  let err = divide(5, 0)
  print(ok)
  print(err)
}
