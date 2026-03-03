// Classes
class Animal {
  name: string
  age: integer 32

  new(name: string, age: integer 32) {
    this.name = name
    this.age = age
  }

  greet() = "Hello, I'm {name}"

  virtual birthday() = this with { age: age + 1 }
}

class Dog extends Animal {
  breed: string

  new(name: string, age: integer 32, breed: string) {
    super(name, age)
    this.breed = breed
  }

  override greet() = "Woof! I'm {name}, a {breed}"
}

function main() {
  let animal = Animal("Bob", 3)
  let dog = Dog("Fido", 5, "Golden Retriever")
  print(animal.greet())
  print(dog.greet())
}
