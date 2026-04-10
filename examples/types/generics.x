function identity(x: integer) -> integer = x

let a = identity(42)

println("identity(42) = " + a)

function double(x: integer) -> integer = x * 2

let doubled = double(21)
println("double(21) = " + doubled)

function add(x: integer, y: integer) -> integer = x + y

let sum = add(10, 20)
println("add(10, 20) = " + sum)

function combineStr(a: string, b: string) -> string = a + b

let combined = combineStr("Hello, ", "World!")
println("combined: " + combined)
