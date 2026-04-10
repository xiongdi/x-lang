// @test basic functions
// @stdout: 15
// @stdout: Hello, X!
// @stdout: 120

function add(a: integer, b: integer) -> integer = a + b

let sum = add(10, 5)
println(sum)

function greet(name: string) {
    println("Hello, " + name + "!")
}

greet("X")

function factorial(n: integer) -> integer {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

let fact5 = factorial(5)
println(fact5)

function identity(x) = x
println(identity(42))

function noop() {
}

noop()
println("noop called")
