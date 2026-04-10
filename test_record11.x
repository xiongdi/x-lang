module test

record Simple {
    a: Int,
    b: Int,
    c: Int,
}

function make_simple() -> Simple {
    Simple {
        a: 0,
        b: 1,
        c: 2,
    }
}

println("test")
