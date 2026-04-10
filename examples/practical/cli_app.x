println("CLI App Demo")
println("============")

let args = ["--help", "greet", "World"]

function showHelp() {
    println("Usage: app <command> [options]")
    println("")
    println("Commands:")
    println("  greet <name>  Greet someone")
    println("  calc <a> <op> <b>  Calculate")
    println("")
    println("Options:")
    println("  --help  Show this help")
}

function greet(name: string) {
    println("Hello, " + name + "!")
}

function calc(a: integer, op: string, b: integer) -> integer {
    if op == "+" {
        a + b
    } else if op == "-" {
        a - b
    } else if op == "*" {
        a * b
    } else {
        0
    }
}

let hasHelp = false
for arg in args {
    if arg == "--help" {
        hasHelp = true
    }
}

if hasHelp {
    showHelp()
} else {
    if args[0] == "greet" {
        greet(args[1])
    } else if args[0] == "calc" {
        let result = calc(10, "+", 5)
        println("10 + 5 = " + result)
    }
}
