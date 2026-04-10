class Point {
    private let x: float
    private let y: float

    public new(x: float, y: float) {
        this.x = x
        this.y = y
    }

    public function getX() -> float = this.x
    public function getY() -> float = this.y

    public function show() -> string {
        "Point(" + this.x + ", " + this.y + ")"
    }
}

class Rectangle {
    private let width: float
    private let height: float

    public new(width: float, height: float) {
        this.width = width
        this.height = height
    }

    public function area() -> float = this.width * this.height

    public function isSquare() -> boolean = this.width == this.height
}

let p1 = Point(3.0, 4.0)
let p2 = Point(0.0, 0.0)

println("Point 1: " + p1.show())
println("Point 1 X: " + p1.getX())
println("Point 1 Y: " + p1.getY())

let rect = Rectangle(5.0, 3.0)
println("Rectangle area: " + rect.area())
println("Is square: " + rect.isSquare())

let square = Rectangle(4.0, 4.0)
println("Square area: " + square.area())
println("Is square: " + square.isSquare())
