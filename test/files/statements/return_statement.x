function find_max(a: Int, b: Int) -> Int {
  if a > b {
    return a
  }
  return b
}

function main() {
  let result = find_max(10, 20)
  println(result)
}
