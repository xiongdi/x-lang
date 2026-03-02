// mandelbrot: Mandelbrot set (Benchmarks Game)
fun mandel(cr, ci, zr, zi, n) {
  if n <= 0 {
    return 0
  }
  let zr2 = zr * zr
  let zi2 = zi * zi
  if zr2 + zi2 > 4 {
    return n
  }
  let nzr = zr2 - zi2 + cr
  let nzi = 2 * zr * zi + ci
  return mandel(cr, ci, nzr, nzi, n - 1)
}

fun main() {
  let k = mandel(0, 0, 0, 0, 20)
  print(k)
}
