use actix_web::{ 
    post,
    HttpResponse,
    web
};
use bson::oid::ObjectId;
use bson::DateTime;
use futures::TryStreamExt;
use mongodb::options::FindOneOptions;
use mongodb::options::FindOptions;
use serde::Deserialize;
use serde::Serialize;

use mongodb::{bson::doc, results::InsertOneResult, Collection, Database};

use crate::model::user_vocab::{Status, UserVocab};
use crate::model::user_vocab_list::UserVocabList;
use crate::model::status_list::StatusList;
use crate::model::vocab_list::{self, VocabList};

const COL_NAME: &str = "user_vocab_list";
const COL_NAME_VOCAB: &str = "vocab_list";

#[derive(Deserialize)]
pub struct MyObj {
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
async fn create_vocab_link(db: web::Data<Database>, myObj: web::Json<MyObj>) -> HttpResponse {
 
    let user_id = myObj.user_id;
    let collection:  Collection<UserVocabList> = db.collection(COL_NAME);
    let collection_vocab: Collection<VocabList> = db.collection(COL_NAME_VOCAB);
    let test: Vec<UserVocab>;

    let options = FindOneOptions::builder()
        .projection(doc! {"vocab_list._id": 1})
        .build();

    let result = collection_vocab
        .clone_with_type::<VocabListResult>()
        .find_one(doc!{"name": "n5"}, options)
        .await;
    println!("{:?}", result);  

            
    let link_vocab_list = UserVocabList { 
        user_id: user_id, 
        vocab_list: vec![
           StatusList{
                status_list_id: myObj.vocab_list_id,
                user_vocabs: vec![
                    UserVocab {
                        // change this to all word ids 
                        word_id: user_id,
                        priority: DateTime::now(),
                        status: Status::New, 
                    }
                ] 
            },

        ],
    };
    HttpResponse::Ok().body("bitch")
    //let result = collection
     //   .insert_one(link_vocab_list, None)
       // .await;
    //match result {
      //  Ok(InsertOneResult{ inserted_id: user_vocablist, ..}) => HttpResponse::Ok().json(user_vocablist),
        //Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    // }
}

