use actix_web::{
    get, 
    post,
    HttpResponse,
    web
};
use mongodb::{bson::doc, Collection, Database};

use crate::model::user::User;

const COL_NAME: &str = "user";

#[get("/get_user/{username}")]
async fn get_user(db: web::Data<Database>, username: web::Path<String>) -> HttpResponse {

    let username = username.into_inner();
    let collection: Collection<User> = db.collection(COL_NAME);

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
async fn create_user(db: web::Data<Database>, form: web::Form<User>) -> HttpResponse {
    let collection = db.collection(COL_NAME);
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[derive(Deserialize)]
pub struct UserLogin{
    user_name: String,
    password: String
}

#[post("/login")]
async fn login(db: web::Data<Database>, credentials: web::Json<UserLogin> ) -> HttpResponse {
    let collection = db.collection(COL_NAME);
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }


    let collection: Collection<User> = db.collection(COL_NAME);

    match collection
        .find_one(doc! { "username": credentials.username }, None)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No user found with username {username}"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }

    

}
