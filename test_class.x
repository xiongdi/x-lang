class Person {
    let name: string
    let age: integer

    function greet() {
        println(self.name)
    }

    function get_age() -> integer {
        return self.age
    }
}

let p = Person("Alice", 30)
p.greet()
let age = p.get_age()
println(age)
