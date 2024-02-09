mod api;
mod model;
mod repository;

use actix_web::{App, HttpServer, web};
use api::user::{get_user, create_user};
use mongodb::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    
    let uri = "mongodb+srv://jeffreyvanderzande:bPXGXWYgdCrO8fGm@testdb.thdjsuq.mongodb.net/?retryWrites=true&w=majority";
    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(create_user)
            .service(get_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
