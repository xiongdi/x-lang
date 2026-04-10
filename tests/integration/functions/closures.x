// @test closures
// @stdout: 8
// @stdout: 13

function makeAdder(n: integer) {
    function(x: integer) -> x + n
}

let add5 = makeAdder(5)
let add10 = makeAdder(10)

println(add5(3))
println(add10(3))

let base = 100
let addBase = function(x) -> x + base

println(addBase(50))

function makeCounter() {
    let mutable count = 0
    function() {
        count = count + 1
        count
    }
}

let counter = makeCounter()
println(counter())
println(counter())
println(counter())
