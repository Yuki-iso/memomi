use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

use super::status_list::StatusList;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserVocabList{
    pub user_id: ObjectId,
    pub status_list: Vec<StatusList>,
}
