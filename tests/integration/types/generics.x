// @test generics
// @stdout: 42
// @stdout: hello

function identity(x) = x

let num = identity(42)
println(num)

let str = identity("hello")
println(str)

function first(items) = items[0]

let numbers = [1, 2, 3]
println(first(numbers))

let words = ["a", "b", "c"]
println(first(words))

function pair(a, b) = [a, b]

let p = pair(1, "one")
println(p[0])
println(p[1])
