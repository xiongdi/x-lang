println("State Effect Demo")
println("=================")

let mutable counter = 0

counter = counter + 1
println("Counter: " + counter)

counter = counter + 1
println("Counter: " + counter)

counter = counter * 2
println("Counter doubled: " + counter)

let mutable sum = 0
for i in 1..=10 {
    sum = sum + i
}
println("Sum of 1-10: " + sum)
