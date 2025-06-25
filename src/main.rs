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