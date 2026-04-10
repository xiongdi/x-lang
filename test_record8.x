module test

record List {
    data: [Int],
    length: Int,
    capacity: Int,
}

function empty() -> List {
    List {
        data: [],
        length: 0,
        capacity: 0,
    }
}

println("test")
