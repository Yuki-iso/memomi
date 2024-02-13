use actix_web::{
    get, 
    // post,
    HttpResponse,
    web
};
use mongodb::{bson::doc, Collection, Database};
use crate::model::user_wordlist::UserWordList;

const COL_NAME: &str = "user_wordlist";

#[get("/get_wordlist/{user_id}")]
async fn get_user_wordlist(db: web::Data<Database>, user_id: web::Path<String>) -> HttpResponse {

    let user_id = user_id.into_inner();
    let collection: Collection<UserWordList> = db.collection(COL_NAME);

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

