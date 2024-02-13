use actix_web::{ 
    post,
    HttpResponse,
    web
};
use serde::Deserialize;

//use mongodb::{bson::doc, Collection, Database};
// use crate::model::user_wordlist::UserWordList;

const COL_NAME: &str = "user_vocab_list";

#[derive(Deserialize)]
pub struct MyObj {
    userId: String,
    vocab_listId: String
}



#[post("/create_vocab_link")]
async fn create_vocab_link(/*db: web::Data<Database>, */myObj: web::Json<MyObj>) -> HttpResponse {

    println!("test");
    println!("User ({}) and vocab ({}) got", myObj.userId, myObj.vocab_listId);

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


    match result {
    HttpResponse::Ok().body("user added")
       Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

