let double = function(x) -> x * 2
let add = function(x, y) -> x + y

println("double(5) = " + double(5))
println("add(3, 4) = " + add(3, 4))

let numbers = [1, 2, 3, 4, 5]

let first = numbers[0]
let second = numbers[1]

println("First: " + first)
println("Second: " + second)

let isPositive = function(x) -> x > 0

println("Is 5 positive? " + isPositive(5))
println("Is -3 positive? " + isPositive(-3))
