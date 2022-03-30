use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum PlayerRequest {
    SetName {
        name: String,
    }, 
    Chat {
        msg: String,
    },
    ImReady,
    ImUnready,
    Chunk {
        bin: Vec<u8>
    },
    Mark {
        score: i8,
    },
    Lexicon(Vec<String>),
    LexiconService(u32),
    LexiconGit(String),
}
