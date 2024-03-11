use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

use super::user_vocab::UserVocab;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct StatusList{
    pub status_list_id: ObjectId,
    pub user_vocabs: Vec<UserVocab>, 
    pub vocab_list_id: ObjectId,
}
