// @test Option and Result types
// @stdout: Some: 5
// @stdout: None
// @stdout: Division result: 5
// @stdout: Division by zero

enum Option<T> {
    Some(T)
    None
}

let some_val = Option.Some(5)
let none_val = Option.None

when some_val is {
    Option.Some(v) => println("Some: " + v)
    Option.None => println("None")
}

when none_val is {
    Option.Some(v) => println("Some: " + v)
    Option.None => println("None")
}

function divide(a: integer, b: integer) -> Option<integer> {
    if b == 0 {
        Option.None
    } else {
        Option.Some(a / b)
    }
}

let result1 = divide(10, 2)
let result2 = divide(10, 0)

when result1 is {
    Option.Some(v) => println("Division result: " + v)
    Option.None => println("Division by zero")
}

when result2 is {
    Option.Some(v) => println("Division result: " + v)
    Option.None => println("Division by zero")
}
