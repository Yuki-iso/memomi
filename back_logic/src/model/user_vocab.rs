use serde::{Deserialize, Serialize};
use bson::{oid::ObjectId, DateTime};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Status{
    New, 
    Failed,
    Review,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserVocab{
    pub word_id: ObjectId,
    pub priority: DateTime,
    pub status: Status, 
}
