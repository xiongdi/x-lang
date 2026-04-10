enum Color {
    Red
    Green
    Blue
}

enum Option<T> {
    Some(T)
    None
}

let red = Color.Red
let green = Color.Green
let blue = Color.Blue

let someValue = Option.Some(42)
let noneValue = Option.None

when someValue is {
    Option.Some(v) => println("Some value: " + v)
    Option.None => println("No value")
}

when noneValue is {
    Option.Some(v) => println("Some value: " + v)
    Option.None => println("No value")
}
