enum Option<T> {
    Some(T)
    None
}

let opt = Option.Some(42)

when opt is {
    Option.Some(value) => println(value)
    Option.None => println("No value")
}

let opt2 = Option.None

when opt2 is {
    Option.Some(value) => println(value)
    Option.None => println("No value")
}
