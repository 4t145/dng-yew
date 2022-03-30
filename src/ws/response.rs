
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerState {
    pub name: String,
    pub idx: u8,
    pub ready: bool,
    pub score: [u8;3],
    pub drawpoint: u8,
    pub timepoint: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerResponse {
    Chat {
        sender: String,
        msg: String
    },
    Notice {
        msg: String
    },
    Warn {
        msg: String
    },
    GameStart,
    GameEnd,
    Topic {
        topic_word: String
    },
    Chunk {
        bin: Vec<u8>
    },

    TurnStart(u8),
    TurnEnd,
    MarkStart,
    Poll,
    MarkEnd,

    PlayerStates (Vec<PlayerState>),
    CountDown(u8),

    RoomFullfilled
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum Stage {
//     Unready,
//     Ready,
//     Drawing(usize),
//     Marking(usize),
//     Over
// }

// #[derive(Debug, Clone, Serialize, Deserialize, Default)]
// pub struct RoomState {
//     pub name: Option<String>,
//     pub stage: String,
//     pub playercount: u8,
//     pub user_lexicon: bool,
//     pub lexicon: u32,
// }