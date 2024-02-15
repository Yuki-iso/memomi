use serde::{Deserialize, Serialize};

use super::vocab::Vocab;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct VocabList{
    pub name: String, 
    pub owner: String, 
    pub public: bool,
    pub vocab_list: Vec<Vocab>,
}
