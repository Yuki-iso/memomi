use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct VocabList{
    pub vocab_list_id: ObjectId,
}
