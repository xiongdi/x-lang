println("Parallel Computing Demo")
println("=======================")

function sumRange(start: integer, end: integer) -> integer {
    let mutable total = 0
    for i in start..end {
        total = total + i
    }
    total
}

let sum1 = sumRange(1, 100)
let sum2 = sumRange(100, 200)
let sum3 = sumRange(200, 300)
let sum4 = sumRange(300, 400)

let total = sum1 + sum2 + sum3 + sum4
println("Parallel-like sum (1-400): " + total)

let direct = sumRange(1, 400)
println("Direct sum (1-400): " + direct)
