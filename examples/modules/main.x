import utils;
import types;
import helpers.math;
import helpers.string;

function main() {
    println("=== Module System Demo ===")
    println("")
    
    println("Using utils module:")
    let sum = add(10, 20)
    println("  add(10, 20) = ")
    println(sum)
    
    let product = multiply(5, 6)
    println("  multiply(5, 6) = ")
    println(product)
    
    println("  PI = ")
    println(PI)
    println("")
    
    println("Using helpers.math module:")
    let sq = square(7)
    println("  square(7) = ")
    println(sq)
    
    let cb = cube(3)
    println("  cube(3) = ")
    println(cb)
    
    let fact = factorial(5)
    println("  factorial(5) = ")
    println(fact)
    println("")
    
    println("Using helpers.string module:")
    let greeting = greet("World")
    println("  greet(\"World\") = ")
    println(greeting)
    
    let repeated = repeat("X", 5)
    println("  repeat(\"X\", 5) = ")
    println(repeated)
    println("")
    
    println("Using types module:")
    let p = Point { x: 3.14, y: 2.71 }
    println("  Point { x: 3.14, y: 2.71 }")
    println("  p.x = ")
    println(p.x)
    println("  p.y = ")
    println(p.y)
    println("")
    
    println("=== Module Demo Complete ===")
}
