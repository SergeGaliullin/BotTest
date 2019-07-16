use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMessage {
    pub text: String,
    pub chat_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMessageChat {
    pub id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestMessage {
    pub text: String,
    pub chat: ResponseMessageChat,
}
