module test

record Simple {
    data: [Int],
    len: Int,
    cap: Int,
}

function make_simple() -> Simple {
    Simple { data: [], len: 0, cap: 0 }
}

println("test")
