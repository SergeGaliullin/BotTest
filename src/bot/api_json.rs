use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMessage {
    pub text: String,
    pub chat_id: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RequestMessage {
    pub message: Message,
    pub chat: Chat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    pub id: String
}

pub struct Message {
    pub text: String
}
