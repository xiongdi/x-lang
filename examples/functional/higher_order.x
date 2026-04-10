function applyTwice(f, x) {
    f(f(x))
}

function compose(f, g) {
    function(x) -> f(g(x))
}

let double = function(x) -> x * 2
let increment = function(x) -> x + 1

let result1 = applyTwice(double, 3)
println("applyTwice(double, 3) = " + result1)

let result2 = applyTwice(increment, 5)
println("applyTwice(increment, 5) = " + result2)

let doubleThenIncrement = compose(increment, double)
println("doubleThenIncrement(5) = " + doubleThenIncrement(5))

function mapManual(list, f) {
    let result = []
    for item in list {
        result = result + [f(item)]
    }
    result
}

let numbers = [1, 2, 3, 4, 5]
let doubled = mapManual(numbers, double)
println("Doubled: " + doubled)
