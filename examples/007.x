trait Flyable {
    function fly()
}

class Bird {
    function fly() {
        println("Flying")
    }
}

implement Flyable for Bird {
    function fly() {
        println("Flying")
    }
}
let bird = Bird()
bird.fly()
