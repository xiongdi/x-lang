type Shape =
  | Circle { radius: Float }
  | Rect { width: Float, height: Float }
  | Point

function main() {
  let circle: Shape = Circle { radius: 5.0 }
  let rect: Shape = Rect { width: 10.0, height: 20.0 }
  let point: Shape = Point
}
