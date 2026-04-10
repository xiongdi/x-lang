println("Channel Demo")
println("============")
println("Note: Full channel support coming soon")

let producer = [1, 2, 3, 4, 5]
let consumer = []

for item in producer {
    consumer = consumer + [item * 2]
}

println("Produced: " + producer)
println("Consumed (doubled): " + consumer)
