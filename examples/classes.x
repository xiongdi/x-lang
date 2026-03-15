// Class and trait system test for X language

// Basic class declaration
class Point {
    x: Int;
    y: Int;

    new(x: Int, y: Int) {
        this.x = x;
        this.y = y;
    }

    function getX() -> Int {
        return this.x;
    }

    function getY() -> Int {
        return this.y;
    }
}

// Trait declaration
trait Drawable {
    function draw() -> Unit;
}

// Class implementing a trait
class Circle implement Drawable {
    x: Int;
    y: Int;
    radius: Int;

    new(x: Int, y: Int, radius: Int) {
        this.x = x;
        this.y = y;
        this.radius = radius;
    }

    function draw() -> Unit {
        println("Drawing circle");
    }
}

function main() {
    println("Class test passed");
}
