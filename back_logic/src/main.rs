mod api;
mod model;

use actix_web::{middleware, web, App, HttpServer};
use api::user::{get_user, create_user, login};
use api::user_vocab_list::{create_vocab_link, new_words, update_status};
use mongodb::Client;
use std::env;
extern crate dotenv;
use  dotenv::dotenv;
use biscuit_actix_middleware::BiscuitMiddleware;
extern crate biscuit_auth as biscuit;
use biscuit::{KeyPair};

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
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");  
    let db = client.database(db_name.as_str());

    let root = KeyPair::new();

    //println!("yeah {}", root.public());

    let public_key = root.public();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(BiscuitMiddleware::new(public_key))
            .app_data(web::Data::new(db.clone()))
            .service(create_user)
            .service(get_user)
            .service(create_vocab_link)
            .service(new_words)
            .service(update_status)
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
