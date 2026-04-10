// @test enum types
// @stdout: Red
// @stdout: Green
// @stdout: Blue
// @stdout: Some: 42
// @stdout: None

enum Color {
    Red
    Green
    Blue
}

let red = Color.Red
let green = Color.Green
let blue = Color.Blue

println(red)
println(green)
println(blue)

enum Option<T> {
    Some(T)
    None
}

let some = Option.Some(42)
let none = Option.None

when some is {
    Option.Some(v) => println("Some: " + v)
    Option.None => println("None")
}

when none is {
    Option.Some(v) => println("Some: " + v)
    Option.None => println("None")
}
