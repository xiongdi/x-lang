// Pattern matching with match
type Shape =
  | Circle  { radius: Float }
  | Rect    { width: Float, height: Float }
  | Point

function area(shape: Shape): float =
  match shape {
    Circle { radius }         => 3.14159 * radius ^ 2
    Rect   { width, height }  => width * height
    Point                      => 0.0
  }

function grade(score: integer 32): string =
  match score {
    s if s >= 90 => "A"
    s if s >= 80 => "B"
    s if s >= 70 => "C"
    s if s >= 60 => "D"
    _            => "F"
  }

function main() {
  let circle = Circle { radius: 5.0 }
  let a = area(circle)
  let g = grade(85)
  print(a)
  print(g)
}
