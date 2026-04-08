class Rectangle {
    private let width: integer
    private let height: integer

    constructor(width: integer, height: integer) {
        self.width = width
        self.height = height
    }

    function area() -> integer {
        self.width * self.height
    }
}

let rect = Rectangle(77, 88)
println(rect.area())
