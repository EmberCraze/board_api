use crate::tictactoe::serializers;
use rocket::serde::json::Json;


#[get("/", format = "json", data = "<message>")]
pub fn index(message: Json<serializers::Message>) -> Option<Json<serializers::Message>> {
    Some(Json(serializers::Message {
            id: message.id+1,
            message: [&message.message, " Success!"].join(""),
        }))
    
}

