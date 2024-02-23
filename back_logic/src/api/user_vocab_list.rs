use std::borrow::Borrow;
use std::borrow::BorrowMut;

use actix_web::{ 
    post,
    HttpResponse,
    web,
    get,
    
};
use bson::oid::ObjectId;
use bson::DateTime;
use mongodb::options::{FindOneOptions, FindOptions};
use serde::Deserialize;
use serde::Serialize;
use actix_web::web::Json;
use mongodb::{bson::doc, results::InsertOneResult, Collection, Database};

use crate::model::user_vocab::{Status, UserVocab};
use crate::model::user_vocab_list::UserVocabList;
use crate::model::status_list::StatusList;
use crate::model::vocab_list::VocabList;

const COL_NAME: &str = "user_vocab_list";
const COL_NAME_VOCAB: &str = "vocab_list";

#[derive(Deserialize)]
pub struct UserInput{
    user_id: ObjectId,
    vocab_list_id: ObjectId
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct VocabListResult{
    pub vocab_list: Vec<VocabResult>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct VocabResult {
    pub _id: ObjectId,
}

#[post("/create_vocab_link")]
async fn create_vocab_link(db: web::Data<Database>, user_input: web::Json<UserInput>) -> HttpResponse {
 
    let collection:  Collection<UserVocabList> = db.collection(COL_NAME);
    let collection_vocab: Collection<VocabList> = db.collection(COL_NAME_VOCAB);

    let options = FindOneOptions::builder()
        .projection(doc! {"vocab_list._id": 1})
        .build();

    let result = collection_vocab
        .clone_with_type::<VocabListResult>()
        .find_one(doc!{"name": "n5"}, options)
        .await.unwrap();
    let vocab_list_array = result.unwrap().vocab_list;
    let mut user_vocabs: Vec<UserVocab> = Vec::new();
    
    for i in vocab_list_array{
        user_vocabs.push(UserVocab {
            word_id: i._id,
             priority: DateTime::now(),
            status: Status::New,
        })
    };
               
    let link_vocab_list = UserVocabList { 
        user_id: user_input.user_id, 
        status_list: vec![
           StatusList{
                status_list_id: user_input.vocab_list_id,
                user_vocabs: user_vocabs
            },

        ],
    };
    let result = collection
        .insert_one(link_vocab_list, None)
        .await;
    match result {
        Ok(InsertOneResult{ inserted_id: user_vocablist, ..}) => HttpResponse::Ok().json(user_vocablist),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
     }
}

#[get("/new_words/{status_list_id}")]
async fn new_words(db: web::Data<Database>, status_list_id: web::Path<String>) -> HttpResponse {

    let collection: Collection<UserVocabList> = db.collection("user_vocab_list");

    let filter = doc! {
        "status_list": {
            "$elemMatch": { 
                "status_list_id": mongodb::bson::oid::ObjectId::parse_str(status_list_id.to_string()).unwrap()
            }
        }
    };
    
    let mut result = collection.find_one(filter, None).await.unwrap();
    let mut words: Vec<UserVocab> = Vec::new();
    let mut i = 0;

    while result.as_mut().unwrap().status_list[0].user_vocabs.len() > 0 {
        let temp = result.as_mut().unwrap().status_list[0].user_vocabs.pop().unwrap();
        match temp.status {
            Status::New => if i < 10 {words.push(temp); i = i + 1;},
            _ => words.push(temp),
        }
    }   
    HttpResponse::Ok().json(words)
}
