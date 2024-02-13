use actix_web::{ 
    post,
    HttpResponse,
    web
};
use bson::oid::ObjectId;
use serde::Deserialize;

use mongodb::{bson::doc, results::InsertOneResult, Collection, Database};

use crate::model::user_vocab_list::UserVocabList;

const COL_NAME: &str = "user_vocab_list";

#[derive(Deserialize)]
pub struct MyObj {
    user_id: ObjectId,
    vocab_list_id: ObjectId,
}



#[post("/create_vocab_link")]
async fn create_vocab_link(db: web::Data<Database>, myObj: web::Json<MyObj>) -> HttpResponse {

    println!("test");
    println!("User ({}) and vocab ({}) got", myObj.user_id, myObj.vocab_list_id);
    
    let user_id = myObj.user_id;
    let collection:  Collection<UserVocabList> = db.collection(COL_NAME);
    let doc = UserVocabList {
        user_id: user_id, 
        vocab_list: [
            {"vocab_list_id": myObj.vocab_list_id}
        ],
    };
    match collection
        .insert_one(doc, None)
        .await
    {
        Ok(InsertOneResult(user_vocablist)) => HttpResponse::Ok().json(user_vocablist),
        Ok(InsertOneResult) => {
            HttpResponse::NotFound().body(format!("User has no word list"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    };

    return HttpResponse::Ok().body("testing");
}

