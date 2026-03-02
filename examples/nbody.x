// n-body: N-body simulation (Benchmarks Game)
// Minimal: 2 bodies, 5 steps via recursion
fun step(n, x1, v1, x2, v2) {
  if n <= 0 {
    return x1
  }
  let dx = x2 - x1
  let a = dx
  let nv1 = v1 + a
  let nv2 = v2 - a
  let nx1 = x1 + nv1
  let nx2 = x2 + nv2
  return step(n - 1, nx1, nv1, nx2, nv2)
}

fun main() {
  let x = step(5, 0, 0, 100, 0)
  print(x)
}
