module helpers.string;

export greet;
export repeat;

function greet(name: string) -> string {
    "Hello, " + name + "!"
}

function repeat(s: string, times: integer) -> string {
    let result = ""
    let i = 0
    while i < times {
        result = result + s
        i = i + 1
    }
    result
}
