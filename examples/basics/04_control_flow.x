let score = 85

if score >= 90 {
    println("Grade: A")
} else if score >= 80 {
    println("Grade: B")
} else if score >= 70 {
    println("Grade: C")
} else {
    println("Grade: F")
}

let mutable count = 0
while count < 5 {
    println("Count: " + count)
    count += 1
}

let numbers = [1, 2, 3, 4, 5]

for num in numbers {
    println("Number: " + num)
}

for i in 0..5 {
    println("Index: " + i)
}

let mutable total = 0
for i in 1..=10 {
    total += i
}
println("Sum 1-10: " + total)
