println("HTTP Server Demo")
println("================")
println("Note: Full HTTP server support coming soon")

class Request {
    private let method: string
    private let path: string

    public new(method: string, path: string) {
        this.method = method
        this.path = path
    }

    public function getMethod() -> string = this.method
    public function getPath() -> string = this.path
}

class Response {
    private let status: integer
    private let body: string

    public new(status: integer, body: string) {
        this.status = status
        this.body = body
    }

    public function getStatus() -> integer = this.status
    public function getBody() -> string = this.body
}

function handleRequest(req: Request) -> Response {
    if req.getPath() == "/" {
        Response(200, "Welcome!")
    } else if req.getPath() == "/api" {
        Response(200, "{\"status\": \"ok\"}")
    } else {
        Response(404, "Not Found")
    }
}

let req1 = Request("GET", "/")
let req2 = Request("GET", "/api")
let req3 = Request("GET", "/unknown")

let resp1 = handleRequest(req1)
let resp2 = handleRequest(req2)
let resp3 = handleRequest(req3)

println("Response 1: " + resp1.getStatus() + " - " + resp1.getBody())
println("Response 2: " + resp2.getStatus() + " - " + resp2.getBody())
println("Response 3: " + resp3.getStatus() + " - " + resp3.getBody())
