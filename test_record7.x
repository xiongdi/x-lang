module test

record List<T> {
    data: [T],
    length: Int,
    capacity: Int,
}

function empty<T>() -> List<T> {
    List {
        data: [],
        length: 0,
        capacity: 0,
    }
}

println("test")
