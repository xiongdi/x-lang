// @test basic pattern matching
// @stdout: one
// @stdout: two
// @stdout: many

function describe(n: integer) -> string {
    when n is {
        1 => "one"
        2 => "two"
        _ => "many"
    }
}

println(describe(1))
println(describe(2))
println(describe(5))

let x = 42
when x is {
    0 => println("zero")
    42 => println("the answer")
    _ => println("something else")
}

let name = "X"
when name is {
    "X" => println("X language")
    _ => println("unknown")
}
