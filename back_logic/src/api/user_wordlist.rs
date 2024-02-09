use actix_web::{
    get, 
    // post,
    HttpResponse,
    web
};
use mongodb::{Client, Collection, bson::doc};
use crate::model::user_wordlist::UserWordList;

const DB_NAME: &str = "mydb";
const COL_NAME: &str = "user_wordlist";

#[get("/get_wordlist/{user_id}")]
async fn get_user_wordlist(client: web::Data<Client>, user_id: web::Path<String>) -> HttpResponse {

    let user_id = user_id.into_inner();
    let collection: Collection<UserWordList> = client.database(DB_NAME).collection(COL_NAME);

    match collection
        .find_one(doc! { "user_id": &user_id }, None)
        .await
    {
        Ok(Some(wordlist)) => HttpResponse::Ok().json(wordlist),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("User has no word list"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//#[post("/init_wordlist")]
// async fn init_wordlist(client: web::Data<Client>) {}
