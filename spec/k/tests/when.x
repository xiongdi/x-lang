// Pattern matching with when/is
type Shape =
  | Circle  { radius: Float }
  | Rect    { width: Float, height: Float }
  | Point

fun area(shape: Shape): Float =
  when shape is
    Circle { radius }        -> 3.14159 * radius ^ 2
    Rect   { width, height } -> width * height
    Point                    -> 0.0

fun grade(score: Int): String =
  when score is
    s where s >= 90 -> "A"
    s where s >= 80 -> "B"
    s where s >= 70 -> "C"
    s where s >= 60 -> "D"
    _               -> "F"

fun main() {
  let circle = Circle { radius: 5.0 }
  let a = area(circle)
  let g = grade(85)
  print(a)
  print(g)
}
