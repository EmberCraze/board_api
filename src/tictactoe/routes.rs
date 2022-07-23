use crate::tictactoe::serializers;
use rocket::serde::json::Json;


#[get("/", format = "json", data = "<message>")]
pub fn index(message: Json<serializers::Message>) -> Option<Json<serializers::Message>> {
    Some(Json(serializers::Message {
            id: message.id+1,
            message: [&message.message, " Success!"].join(""),
        }))
    
}

#[get("/board", format = "json", data = "<game>")]
pub fn board(game: Json<serializers::Game>) -> Option<Json<serializers::Game>> {
    //TODO we should only get an id and fetch the object based on that
    Some(Json(serializers::Game{
            id: game.id+1,
            active_player: [&game.active_player, " Success!"].join(""),
            board: game.board.clone(),
        }))
    
}
