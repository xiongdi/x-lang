// X-Web Framework Example

// Response builders
function html_response(body: string) -> string {
    concat(
        concat("HTTP/1.1 200 OK  Content-Length: ", int_to_string(string_length(body))),
        concat("  ", body)
    )
}

function json_response(json: string) -> string {
    concat(
        concat("HTTP/1.1 200 OK  Content-Length: ", int_to_string(string_length(json))),
        concat("  ", json)
    )
}

// Main entry point
function main() {
    println("X-Web Framework Demo")

    let home = html_response("Welcome to X-Web!")
    println(home)

    let api = json_response("Hello from X-Web!")
    println(api)
}
