struct Rectangle {
    let width: integer
    let height: integer

    public new(width: integer, height: integer) {
        this.width = width
        this.height = height
    }

    public function area() {
        width * height
    }
}

let rect = Rectangle(width: 77, height: 88)
println(rect.area())
