// @test record types
// @stdout: Point(3, 4)
// @stdout: 3
// @stdout: 4
// @stdout: 25

class Point {
    private let x: integer
    private let y: integer

    public new(x: integer, y: integer) {
        this.x = x
        this.y = y
    }

    public function getX() -> integer = this.x
    public function getY() -> integer = this.y

    public function show() -> string {
        "Point(" + this.x + ", " + this.y + ")"
    }
}

let p = Point(3, 4)
println(p.show())
println(p.getX())
println(p.getY())

class Rectangle {
    private let width: integer
    private let height: integer

    public new(width: integer, height: integer) {
        this.width = width
        this.height = height
    }

    public function area() -> integer = this.width * this.height
}

let rect = Rectangle(5, 5)
println(rect.area())
