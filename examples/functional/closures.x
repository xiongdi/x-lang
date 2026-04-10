function makeCounter() {
    let mutable count = 0
    function() {
        count = count + 1
        count
    }
}

let counter = makeCounter()

function makeAdder(n: integer) {
    function(x: integer) -> x + n
}

let add5 = makeAdder(5)
let add10 = makeAdder(10)

println("add5(3) = " + add5(3))
println("add10(3) = " + add10(3))

let base = 100
let addBase = function(x) -> x + base

println("addBase(50) = " + addBase(50))
