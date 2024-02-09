use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserWordList {
    pub user_id: i32,
    pub user_words: [i32; 0],
}
