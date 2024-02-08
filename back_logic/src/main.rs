use actix_web::{get, App, HttpResponse, HttpServer, Responder};

// Import necessary dependencies from Actix Web framework

#[get("/")]
async fn hello() -> impl Responder {
    // Define a handler function for GET requests to the root path ("/")
    // The function returns a type that implements the Responder trait

    HttpResponse::Ok().body("Hello, World!")
    // Return an HTTP response with a status code of 200 (OK)
    // and a response body of "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Entry point of the application

    HttpServer::new(|| {
        // Create a new instance of the HttpServer

        App::new().service(hello)
        // Create a new instance of the App and register the hello function 

    }).bind("0.0.0.0:80")?.run().await
    // Bind the server to the IP address and port
    // Start the server and await its completion
}
