module test

record Simple {
    data: [Int],
    len: Int,
}

function make_simple() -> Simple {
    Simple { data: [], len: 0 }
}

println("test")
