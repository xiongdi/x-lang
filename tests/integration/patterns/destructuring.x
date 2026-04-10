// @test destructuring patterns
// @stdout: 1
// @stdout: 2
// @stdout: 3

let pair = [1, 2]
let [a, b] = pair

println(a)
println(b)

let triple = [10, 20, 30]
let [x, y, z] = triple

println(x)
println(y)
println(z)

class Point {
    private let x: integer
    private let y: integer

    public new(x: integer, y: integer) {
        this.x = x
        this.y = y
    }

    public function getX() -> integer = this.x
    public function getY() -> integer = this.y
}

let p = Point(100, 200)
println(p.getX())
println(p.getY())

let items = [[1, 2], [3, 4], [5, 6]]
for item in items {
    let [first, second] = item
    println(first)
}
