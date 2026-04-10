module test

record Simple {
    x: Int,
    y: Int,
}

function make_simple() -> Simple {
    Simple { x: 0, y: 1 }
}

println("test")
