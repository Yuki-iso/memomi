mod api;
mod model;

use actix_web::{middleware, web, App, HttpServer};
use api::user::{get_user, create_user};
use api::user_wordlist::get_user_wordlist;
use mongodb::Client;
use std::env;
extern crate dotenv;
use  dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
   
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let uri = match env::var("MONGO_URL") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    }; 
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(client.clone()))
            .service(create_user)
            .service(get_user)
            .service(get_user_wordlist)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
