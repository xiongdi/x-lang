function double(x) -> x * 2
function increment(x) -> x + 1
function square(x) -> x * x

let result = square(increment(double(5)))
println("5 |> double |> increment |> square = " + result)

function pipeline(value) {
    let a = double(value)
    let b = increment(a)
    square(b)
}

println("Pipeline result: " + pipeline(5))

let numbers = [1, 2, 3, 4, 5]

let processed = []
for num in numbers {
    let d = double(num)
    processed = processed + [d]
}

println("Processed: " + processed)
