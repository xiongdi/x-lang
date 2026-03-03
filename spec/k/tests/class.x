// Classes
class Animal {
  name: String
  age: Int

  new(name: String, age: Int) {
    this.name = name
    this.age = age
  }

  greet() = "Hello, I'm {name}"

  virtual birthday() = this with { age: age + 1 }
}

class Dog extends Animal {
  breed: String

  new(name: String, age: Int, breed: String) {
    super(name, age)
    this.breed = breed
  }

  override greet() = "Woof! I'm {name}, a {breed}"
}

fun main() {
  let animal = Animal("Bob", 3)
  let dog = Dog("Fido", 5, "Golden Retriever")
  print(animal.greet())
  print(dog.greet())
}
