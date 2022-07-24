use std::borrow::Cow;
use rocket::serde::{Deserialize, Serialize};

// If the serialization and decerialization are identical, we can combine them into a single
// struct. Otherwize use two different structs
#[derive(Serialize, Deserialize)]
pub struct Message<'r> {
    pub id: usize,
    pub message: Cow<'r, str>,
}

#[derive(Serialize, Deserialize)]
pub struct Game<'r>{
    pub id: usize,
    pub active_player: Cow<'r, str>,
    // Defines a board that is of size 3x3 full of strings
    pub board: [[Cow<'r, str>; 3]; 3],
}
