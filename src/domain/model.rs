use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response<T> {
    pub ok: bool,
    pub description: Option<String>,
    pub result: T,
}

pub type GetUpdateResponse = Response<Vec<Update>>;

#[derive(Deserialize, Debug)]
pub struct Update {
    pub update_id: u64,
    pub message: Option<Message>,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub message_id: u64,
    pub chat: Chat,
    pub from: User,
    pub text: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Chat {
    pub id: u64,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: u64,
    pub first_name: String,
}
