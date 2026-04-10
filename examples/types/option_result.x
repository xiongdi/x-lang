enum Option<T> {
    Some(T)
    None
}

function divide(a: integer, b: integer) -> Option<float> {
    if b == 0 {
        Option.None
    } else {
        Option.Some(a as float / b as float)
    }
}

let result1 = divide(10, 2)
let result2 = divide(10, 0)

when result1 is {
    Option.Some(v) => println("10 / 2 = " + v)
    Option.None => println("Division by zero!")
}

when result2 is {
    Option.Some(v) => println("10 / 0 = " + v)
    Option.None => println("Division by zero!")
}

let numbers = [1, 2, 3, 4, 5]

when Option.Some(numbers[0]) is {
    Option.Some(v) => println("First element: " + v)
    Option.None => println("No element")
}
