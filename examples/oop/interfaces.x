interface Flyable {
    function fly() -> string
}

interface Swimmable {
    function swim() -> string
}

class Bird implements Flyable {
    private let name: string

    public new(name: string) {
        this.name = name
    }

    public function fly() -> string = this.name + " is flying high!"
}

class Fish implements Swimmable {
    private let name: string

    public new(name: string) {
        this.name = name
    }

    public function swim() -> string = this.name + " is swimming!"
}

class Duck implements Flyable, Swimmable {
    private let name: string

    public new(name: string) {
        this.name = name
    }

    public function fly() -> string = this.name + " is flying!"
    public function swim() -> string = this.name + " is swimming!"
}

let bird = Bird("Eagle")
let fish = Fish("Nemo")
let duck = Duck("Donald")

println(bird.fly())
println(fish.swim())
println(duck.fly())
println(duck.swim())
