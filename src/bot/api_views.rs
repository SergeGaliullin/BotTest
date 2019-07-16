use super::api_json::{ResponseMessage, RequestMessage};
use actix_web::{web, HttpResponse};

pub fn new_message(message: web::Json<RequestMessage>) -> HttpResponse {   
    HttpResponse::Ok().json(
        ResponseMessage {
            text: message.text.clone(),
            chat_id: message.chat.id.clone()
        }
    )
}
