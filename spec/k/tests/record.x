// Record (struct) operations
type Point = { x: integer 32, y: integer 32 }

function main() {
  let origin = { x: 0, y: 0 }
  let p = { x: 10, y: 20 }
  let q = p with { x: 100 }
  print(p.x)
  print(q.y)
}
