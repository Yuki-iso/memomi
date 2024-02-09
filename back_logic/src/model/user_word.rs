use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserWord {
    pub word_id: i32,
    pub priority: bool, 
    pub status: bool,
}
