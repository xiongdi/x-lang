// Result type
type DivError = DivisionByZero

function divide(a: integer 32, b: integer 32): Result<integer 32, DivError> =
  if b == 0 then
    Err(DivisionByZero)
  else
    Ok(a / b)

function main() {
  let ok = divide(10, 2)
  let err = divide(5, 0)
  print(ok)
  print(err)
}
