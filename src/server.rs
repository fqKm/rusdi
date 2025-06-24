use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use crate::{HttpMethod, Request, StatusCode};
use crate::response::Response;

type RouteHandler = fn(Option<Request>) -> Response;
pub struct Server{
    address: String,
    routes: HashMap<(String, String), RouteHandler>
}

impl Server{
    pub fn create(address: &str) -> Server{
        Server {
            address: address.to_string(),
            routes: HashMap::new()
        }

    }
    pub fn route(&mut self, method: &str, path: &str, route: RouteHandler){
        self.routes.insert((method.to_string(), path.to_string()), route);
    }
    /**
    Still Need Separate for Each HTTP Method 
    **/
    pub fn get(&mut self, path: &str, route: RouteHandler){
        
    }
    pub fn post(&mut self, path: &str, route: RouteHandler){
        
    }
    pub fn put(&mut self, path: &str, route: RouteHandler){
        
    }
    pub fn delete(&mut self, path: &str, route: RouteHandler){
        
    }
    pub fn head(&mut self, path: &str, route: RouteHandler){
        
    }
    pub fn patch(&mut self, path: &str, route: RouteHandler){
        
    }
    pub fn trace(&mut self, path: &str, route: RouteHandler){
        
    }
    pub fn connect(&mut self, path : &str, route: RouteHandler){
        
    }
    pub fn run(self){
        let listener = TcpListener::bind(self.address.as_str()).unwrap();
        let routes = Arc::new(self.routes);
        for stream in listener.incoming(){
            match stream {
                Ok(stream) => {
                    let routes = Arc::clone(&routes);
                    thread::spawn(move ||{
                        handler(stream, &routes);
                    });
                }
                Err(e) => {
                    println!("Connection Failed: {}", e);
                }
            }
        }
    }
}

fn handler(mut stream: TcpStream, routes: &HashMap<(String, String), RouteHandler>) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer){
        Ok(bytes_read) => {
            let raw_request = String::from_utf8_lossy(&buffer[0.. bytes_read]);
            let request = Request::new(&raw_request);
            let key = (request.method.as_str(), request.path.to_string());
            let response = match routes.get(&key){
                Some(handler) => {
                    if request.method == HttpMethod::GET || request.method == HttpMethod::HEAD || request.method == HttpMethod::DELETE {
                        handler(None)
                    } else{
                        handler(Some(request))
                    }
                },
                None => Response::new(StatusCode::NotFound, "Not Found"),
            };
            let _ = stream.write_all(response.to_http_string().as_bytes());
        }
        Err(_) => {
            return;
        }
    }

}