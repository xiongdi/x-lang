// spectral-norm: eigenvalue norm (Benchmarks Game)
fun A(i, j) {
  return 1
}

fun dot(v, i, n, sum) {
  if i >= n {
    return sum
  }
  let a = A(i, 0)
  return dot(v, i + 1, n, sum + a)
}

fun main() {
  let n = 1
  let s = dot(0, 0, n, 0)
  print(s)
}
