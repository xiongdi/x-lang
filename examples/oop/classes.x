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

class Counter {
    private let mutable count: integer

    public new() {
        this.count = 0
    }

    public function increment() {
        this.count = this.count + 1
    }

    public function decrement() {
        this.count = this.count - 1
    }

    public function getValue() -> integer = this.count
}

let p1 = Point(3.0, 4.0)
println("Point: " + p1.show())

let rect = Rectangle(5.0, 3.0)
println("Area: " + rect.area())
println("Is square: " + rect.isSquare())

let counter = Counter()
counter.increment()
counter.increment()
counter.decrement()
println("Counter: " + counter.getValue())
