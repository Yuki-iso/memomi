mod api;
mod model;

use actix_web::{middleware, web, App, HttpServer};
use api::user::{get_user, create_user};
use api::user_vocab_list::{create_vocab_link, update_status};
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
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");  
    let db = client.database(db_name.as_str());

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(db.clone()))
            .service(create_user)
            .service(get_user)
            .service(create_vocab_link)
            .service(update_status)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
