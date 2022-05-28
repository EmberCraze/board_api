use rocket::serde::{Deserialize, Serialize};

// If the serialization and decerialization are identical, we can combine them into a single
// struct. Otherwize use two different structs
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: usize,
    pub message: String,
}
