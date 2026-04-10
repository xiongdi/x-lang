module test

record List {
    data: [Int],
    length: Int,
}

function empty() -> List {
    List {
        data: [],
        length: 0,
    }
}

println("test")
