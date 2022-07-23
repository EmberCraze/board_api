use rocket::serde::{Deserialize, Serialize};

// If the serialization and decerialization are identical, we can combine them into a single
// struct. Otherwize use two different structs
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: usize,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct Game{
    pub id: usize,
    pub active_player: String,
    // Defines a board that is of size 3x3 full of strings
    pub board: [[String; 3]; 3],
}
