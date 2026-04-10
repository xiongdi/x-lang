interface Printable {
    function show() -> string
}

class Point implements Printable {
    private let x: float
    private let y: float

    public new(x: float, y: float) {
        this.x = x
        this.y = y
    }

    public function show() -> string {
        "Point(" + this.x + ", " + this.y + ")"
    }
}

class User implements Printable {
    private let name: string
    private let age: integer

    public new(name: string, age: integer) {
        this.name = name
        this.age = age
    }

    public function show() -> string {
        this.name + " (" + this.age + ")"
    }
}

let p = Point(3.0, 4.0)
let u = User("Alice", 30)

println("Point: " + p.show())
println("User: " + u.show())
