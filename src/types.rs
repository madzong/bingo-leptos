use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub name: String,
    pub size: u8,
    pub fields: Vec<String>,
    pub feedback: bool,
    pub editable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PremadeGamesInfo {
    pub message: String,
    pub games: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseError {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponsePremadeGame {
    pub message: String,
    pub game: String,
}
