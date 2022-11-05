use std::collections::HashMap;

use super::error::{BotError, Result};
use super::messages::*;
use super::model::*;
use crate::domain::user_state::State;
use reqwest::blocking::Client;
use serde_json::json;

const TG_API_PREFIX: &str = "https://api.telegram.org/bot";

pub struct TelegramBotService {
    client: Client,
    token: String,
    processor: MessageProcessor,
}

impl TelegramBotService {
    pub fn new(token: String) -> Self {
        TelegramBotService {
            client: Client::new(),
            token,
            processor: MessageProcessor::new(),
        }
    }

    pub fn handle_updates(&mut self, offset: u64) -> Result<u64> {
        let response = match self.get_updates(offset) {
            Ok(r) => {
                log::debug!("Response result: {}", r.ok);
                if !r.ok {
                    Err(BotError::BadResponseResultError)?;
                }
                r
            }
            Err(e) => Err(e)?,
        };
        let updates_count = response.result.len();
        log::debug!("Count of received updates: {}", updates_count);
        let mut max_update_id = 0;
        for update in response.result {
            if update.update_id > max_update_id {
                max_update_id = update.update_id;
            }
            let answer = &mut self.processor.process_message(update);
            if let Some(answer) = answer {
                self.send_answer(answer.text.as_str(), answer.chat_id)?;
            }
        }
        Ok(max_update_id + 1)
    }

    fn get_updates(&self, offset: u64) -> Result<GetUpdateResponse> {
        let data = json!({ "offset": offset });
        let url = format!("{}{}/getUpdates", TG_API_PREFIX, self.token);
        log::trace!("Request url: {}", url);
        let response = match self
            .client
            .post(url)
            .body(data.to_string())
            .header("content-type", "application/json")
            .send()
        {
            Ok(r) => r,
            Err(e) => {
                log::error!("Error: {}", e);
                Err(BotError::RequestExecutionError(e))?
            }
        };
        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            log::error!("Wrong status code: {}", status);
            return Err(BotError::BadStatusCode(status.as_u16()));
        }
        match response.json::<GetUpdateResponse>() {
            Ok(r) => Ok(r),
            Err(e) => {
                log::error!("Error: {}", e);
                Err(BotError::ParseResponseError(e))
            }
        }
    }

    fn send_answer(&self, text: &str, chat_id: u64) -> Result<()> {
        log::debug!("Start to send answer! Text: {}, chat id: {}", text, chat_id);
        let answer_data = json!({
         "chat_id": chat_id,
         "text": text,
         "parse_mode": "MarkdownV2"
        });
        let url = format!("{}{}/sendMessage", TG_API_PREFIX, self.token);
        log::trace!("Requset url: {}", url);
        match self
            .client
            .post(url)
            .body(answer_data.to_string())
            .header("content-type", "application/json")
            .send()
        {
            Ok(r) => {
                log::trace!("Send response: {:#?}", r);
            }
            Err(e) => {
                log::error!("Error: {}", e);
                Err(BotError::RequestExecutionError(e))?
            }
        };
        Ok(())
    }
}

type StateStore = HashMap<u64, State>;
struct MessageProcessor {
    store: HashMap<u64, String>,
}

struct Answer {
    text: String,
    chat_id: u64,
}

impl Answer {
    fn new(text: String, chat_id: u64) -> Self {
        Self { text, chat_id }
    }
}

impl MessageProcessor {
    fn new() -> Self {
        MessageProcessor {
            store: HashMap::new(),
        }
    }

    fn process_message(&mut self, update: Update) -> Option<Answer> {
        if let Some(message) = update.message {
            log::debug!("Receive message: {:#?}", message);
            if let Some(text) = message.text {
                log::trace!("Receive message text: {}", text);
                let user_name = message.from.first_name;
                let user_id = message.from.id;
                let answer = self.dispatch_text(text.as_str(), user_name, user_id);
                return Some(Answer::new(answer, message.chat.id));
            }
        }
        None
    }

    fn dispatch_text(&mut self, text: &str, user_name: String, user_id: u64) -> String {
        match text {
            HELP => HELP_MESSAGE.to_owned(),
            ABOUT => BOT_DESCRIPTION.to_owned(),
            START => {
                if let Some(v) = self.store.get(&user_id) {
                    return "Тренировка уже идет".to_owned();
                }
                let store = &mut self.store;
                store.insert(user_id, text.to_owned());
                format!("{}, тренировка начата", user_name)
            }
            STOP => {
                if let Some(_) = self.store.get(&user_id) {
                    let store = &mut self.store;
                    store.remove(&user_id);
                    return format!("{}, трунировка окончена", user_name);
                }
                format!("{}, вы не начинали тренировку", user_name)
            }
            _ => format!("{} you send {}", user_name, text),
        }
    }
}
