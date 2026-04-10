module test

record Simple {
    x: Int,
}

function make_simple() -> Simple {
    Simple { x: 0 }
}

println("test")
