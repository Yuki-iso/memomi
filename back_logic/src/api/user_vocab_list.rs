use actix_web::put;
use actix_web::{ 
    post,
    HttpResponse,
    web
};
use bson::oid::ObjectId;
use bson::{DateTime, Document};
use mongodb::options::FindOneOptions;
use serde::Deserialize;
use serde::Serialize;


use mongodb::{bson::doc, results::InsertOneResult, Collection, Database, options::UpdateOptions};

use crate::model::user_vocab::{Status, UserVocab};
use crate::model::user_vocab_list::UserVocabList;
use crate::model::status_list::StatusList;
use crate::model::vocab_list::{VocabList};

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

#[derive(Deserialize, Serialize, Debug)]
pub struct VocabStatus{
    user_id: ObjectId,
    vocab_id: ObjectId,
    status: String,
    priority: String
}



#[put("/update_status")]
async fn update_status(db: web::Data<Database>, status: web::Json<VocabStatus> ) -> HttpResponse {

    let collection: Collection<UserVocabList> = db.collection(COL_NAME);
    let filter = doc! { "user_id": status.user_id };
    let update = doc! { 
        "$set": doc! {
            "status_list.$[].user_vocabs.$[xxx].status": status.status.clone(),
            "status_list.$[].user_vocabs.$[xxx].priority": status.priority.clone()
        }
    }; 

    let array_filters: Option<Vec<Document>> = Some(vec![doc! {"xxx.word_id": status.vocab_id.clone()}]);
    let options = UpdateOptions::builder()
        .array_filters(array_filters)
        .build();

    match collection.update_one(filter, update, Some(options)).await {
        Ok(_) => HttpResponse::Ok().body("Update successful"),
        Err(e) => {
            eprintln!("Error updating status: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

