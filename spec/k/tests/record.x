// Record (struct) operations
type Point = { x: Int, y: Int }

fun main() {
  let origin = { x: 0, y: 0 }
  let p = { x: 10, y: 20 }
  let q = p with { x: 100 }
  print(p.x)
  print(q.y)
}
