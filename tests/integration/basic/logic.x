// @test logical operations

let t = true
let f = false

let and1 = t && t
let and2 = t && f
let or1 = f || t
let or2 = f || f
let not1 = !f

println("true && true: " + and1)
println("true && false: " + and2)
println("false || true: " + or1)
println("false || false: " + or2)
println("!false: " + not1)
