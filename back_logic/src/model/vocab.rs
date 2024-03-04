use serde::{Deserialize, Serialize};
use bson::{oid::ObjectId};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Vocab{
    pub _id: ObjectId,
    pub Kanji: String, 
    pub Kanji_Furigana: String, 
    pub Kana: String,
    pub Pos: String,
    pub English: String,
    pub ex1_ja: String,
    pub ex1_ja_furigana: String,
    pub ex1_en: String,
    pub note: String,
}
