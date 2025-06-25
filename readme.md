# Rusdi is Rust HTTP Framework for building fast, and simple Restful API.

# How To Use?
#### Open your Cargo.toml and add this to your dependencies
```
[dependencies]
rusdi = { git = "https://github.com/fqKm/rusdi", branch = "main" }
```
# How To Create HTTP RestFul API Server?
#### Here is example how to build simple api, to return the request body string
 ```rust
 use rusdi::{ Request, Server, StatusCode, Response};

fn main() {
    // Open Server in address : Localhost, port : 8080 
    let mut server = Server::create("localhost:8080");
    // add route with http method : Post, in path "/api/login" and handle the request with login function
    server.route("POST", "/api/login", login);
    // run server to start accept request
    server.run();
}

// login function to handle Http Request, and Response with HTTP Response
fn login(request:Option<Request>) -> Response {
    // check request
    match request {
        Some(request) => {
            //if there is request sent, throw http_response with Status Code Accepted (202). and send the request body
            Response::new(StatusCode::Accepted, request.body.as_str()) 
        }
        None => {
            //if there is no request sent, throw http_response with status Code Bad Request (400).
            Response::new(StatusCode::BadRequest, "") 
        }
    }
}
```

# Upcoming Work & Features : 
#### 1. Separate Each Route Method
separate each methode to register the route. So it will be like this :
```rust
server.post(path, handler_function); // for post methode
server.get(path, handler_function); // for get methode
server.patch(path, handler_function); // for patch methode
```

#### 2. Add Global Middleware & Each Route Middleware