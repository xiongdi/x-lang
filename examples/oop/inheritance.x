class Animal {
    protected let name: string

    public new(name: string) {
        this.name = name
    }

    public virtual function speak() -> string = "..."

    public function getName() -> string = this.name
}

class Dog extends Animal {
    private let breed: string

    public new(name: string, breed: string) {
        this.name = name
        this.breed = breed
    }

    public override function speak() -> string = "Woof!"

    public function getBreed() -> string = this.breed
}

class Cat extends Animal {
    public new(name: string) {
        this.name = name
    }

    public override function speak() -> string = "Meow!"
}

let dog = Dog("Buddy", "Labrador")
let cat = Cat("Whiskers")

println(dog.getName() + " says: " + dog.speak())
println(cat.getName() + " says: " + cat.speak())
