use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

use super::vocab_list::VocabList;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserVocabList{
    pub user_id: ObjectId,
    pub vocab_list: Vec<VocabList>,
}
