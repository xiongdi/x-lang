// @test enum pattern matching
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

function colorName(c: Color) -> string {
    when c is {
        Color.Red => "Red"
        Color.Green => "Green"
        Color.Blue => "Blue"
    }
}

println(colorName(Color.Red))
println(colorName(Color.Green))
println(colorName(Color.Blue))

enum Option<T> {
    Some(T)
    None
}

let someVal = Option.Some(42)
let noneVal = Option.None

when someVal is {
    Option.Some(v) => println("Some: " + v)
    Option.None => println("None")
}

when noneVal is {
    Option.Some(v) => println("Some: " + v)
    Option.None => println("None")
}

enum Result<T, E> {
    Ok(T)
    Err(E)
}

let okResult = Result.Ok(100)
let errResult = Result.Err("failed")

when okResult is {
    Result.Ok(v) => println("Ok: " + v)
    Result.Err(e) => println("Err: " + e)
}

when errResult is {
    Result.Ok(v) => println("Ok: " + v)
    Result.Err(e) => println("Err: " + e)
}
