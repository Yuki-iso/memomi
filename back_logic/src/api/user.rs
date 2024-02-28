use actix_web::{
    get, 
    post,
    HttpResponse,
    web
};
use mongodb::{bson::doc, Collection, Database};
use serde::Deserialize;
extern crate biscuit_auth as biscuit;

use biscuit::{KeyPair, Biscuit, error};
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
pub struct UserLogin {
    user_name: String,
    password: String
}


#[post("/login")]
async fn login(db: web::Data<Database>, credentials: web::Json<UserLogin> ) -> HttpResponse {

    let public_key = "ed25519/d87cbfe256eafe2aeaa05cbc53f92ce85c2355018725a0dfd2bfbfe0fe298404"; //I pregenerated this but this is bad.

    
    let token1 = {
      // the first block of the token is the authority block. It contains global
      // information like which operation types are available
      let biscuit = biscuit!(r#"
            right("/a/file1.txt", "read");
            right("/a/file1.txt", "write");
            right("/a/file2.txt", "read");
            right("/b/file3.txt", "write");
      "#).build(&root)?; // the first block is signed

      println!("biscuit (authority): {}", biscuit);

      biscuit.to_vec()?
    };

    let collection: Collection<User> = db.collection(COL_NAME);

    match collection
        .find_one(doc! { "username": &credentials.user_name, "password": &credentials.password }, None)
        .await
    {
        Ok(Some(user)) => {
                println!("yeah {}", &user.password);
                HttpResponse::Ok().json(user)
            },
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No user found with username {}", &credentials.user_name))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }

    

}
