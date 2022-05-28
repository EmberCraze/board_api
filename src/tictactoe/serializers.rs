use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct TickTackToe {
    pub id: i32,
    pub plane: Vec<Vec<i32>>,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct NewPlane {
    pub plane: Vec<Vec<i32>>,
    pub title: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct TickTackToeJson {
    pub id: i32,
    pub plane: Vec<Vec<i32>>,
    pub title: String,
    pub description: String,
    // pub created_at: String,
}

impl TickTackToe{
    pub fn to_json(self) -> TickTackToeJson {
        TickTackToeJson {
            id: self.id,
            plane: self.plane,
            title: self.title,
            description: self.description,
            // created_at: self.created_at.format(webserver::DATE_FORMAT).to_string(),
        }
    }
}
