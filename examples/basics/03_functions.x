function add(a: integer, b: integer) -> integer = a + b

function greet(name: string) {
    println("Hello, " + name + "!")
}

function describe(value: integer) -> string {
    if value < 0 {
        "negative"
    } else if value == 0 {
        "zero"
    } else {
        "positive"
    }
}

let sum = add(10, 20)
println("Sum: " + sum)

greet("X")

let desc = describe(42)
println("42 is " + desc)
