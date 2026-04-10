// @test type inference
// @stdout: 42
// @stdout: 3.14
// @stdout: hello
// @stdout: true

let a = 42
let b = 3.14
let c = "hello"
let d = true

println(a)
println(b)
println(c)
println(d)

function add(x, y) = x + y
let result = add(10, 32)
println(result)

function greet(name) {
    "Hello, " + name + "!"
}
let greeting = greet("World")
println(greeting)

let items = [1, 2, 3, 4, 5]
println(items[0])
