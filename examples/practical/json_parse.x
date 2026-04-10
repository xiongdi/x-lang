println("JSON Demo")
println("=========")

let data = {
    "name": "Alice",
    "age": 30,
    "city": "Beijing"
}

println("JSON-like object:")
println("  name: " + data["name"])
println("  age: " + data["age"])
println("  city: " + data["city"])

let items = [
    {"id": 1, "value": "First"},
    {"id": 2, "value": "Second"}
]

println("Array of objects:")
for item in items {
    println("  id: " + item["id"] + ", value: " + item["value"])
}
