use actix_web::{
    get, 
    post,
    HttpResponse,
    web
};
use mongodb::{Client, Collection, bson::doc};

use crate::model::user::User;

const DB_NAME: &str = "mydb";
const COL_NAME: &str = "users";

#[get("/get_user/{username}")]
async fn get_user(client: web::Data<Client>, username: web::Path<String>) -> HttpResponse {

    let username = username.into_inner();
    let collection: Collection<User> = client.database(DB_NAME).collection(COL_NAME);

    match collection
        .find_one(doc! { "username": &username }, None)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No user found with username {username}"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/create_user")]
async fn create_user(client: web::Data<Client>, form: web::Form<User>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(COL_NAME);
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

