module test

record Point {
    x: Int,
    y: Int,
}

function make_point() -> Point {
    Point { x: 1, y: 2 }
}

println("test")
