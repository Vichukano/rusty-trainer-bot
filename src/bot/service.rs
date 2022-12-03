use crate::bot::error::BotError;
use crate::bot::handler::Handler;
use crate::domain::model::{State, UserContext};

use crate::telegram::model::{GetUpdateResponse, Message};
use anyhow::{Context, Result};
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;
use ureq::{Agent, AgentBuilder};

use super::handler::HandlerDispatcher;

const TG_API_PREFIX: &str = "https://api.telegram.org/bot";

struct HttpClient {
    http: Agent,
}

impl HttpClient {
    fn new() -> Self {
        HttpClient {
            http: AgentBuilder::new()
                .timeout_read(Duration::from_secs(5))
                .timeout_connect(Duration::from_secs(20))
                .timeout_write(Duration::from_secs(5))
                .build(),
        }
    }

    fn post(&self, url: String, data: String) -> Result<ureq::Response> {
        self.http
            .post(url.as_str())
            .set("content-type", "application/json")
            .send_string(data.as_str())
            .map_err(|err| err.into())
    }
}

pub struct TelegramBotService {
    token: String,
    http: HttpClient,
    processor: MessageProcessor,
}

impl TelegramBotService {
    pub fn new(token: String) -> Self {
        TelegramBotService {
            token,
            http: HttpClient::new(),
            processor: MessageProcessor::new(),
        }
    }

    pub fn handle_updates(&mut self, offset: u64) -> Result<u64> {
        let response = match self.get_updates(offset) {
            Ok(r) => {
                log::debug!("Response result: {}", r.ok);
                if r.ok != true {
                    Err(BotError::BadResponseResultError)?
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
            //////////!!!!!!!!!!!!!!!!!!!
            if let Some(message) = update.message {
                let chat_id = message.chat.id;
                let answer = self.processor.process_message(message);
                if let Some(answer) = answer {
                    self.send_answer(answer, chat_id)?;
                }
            } else {
                log::debug!("Message is absent in update: {:#?}", update);
            }
        }
        Ok(max_update_id + 1)
    }

    fn get_updates(&self, offset: u64) -> Result<GetUpdateResponse> {
        let data = json!({ "offset": offset });
        let url = format!("{}{}/getUpdates", TG_API_PREFIX, self.token);
        log::trace!("Request url: {}", url);
        let response = match self.http.post(url, data.to_string()) {
            Ok(r) => r,
            Err(e) => {
                log::error!("Error: {}", e);
                Err(e)?
            }
        };
        log::debug!("Response: {:?}", response);
        let status = response.status();
        if status != 200 as u16 {
            log::error!("Wrong status code: {}", status);
            Err(BotError::BadStatusCode(status)).context(format!("Status code: {}", status))?
        }
        match response.into_json() {
            Ok(r) => Ok(r),
            Err(e) => {
                log::error!("Error: {}", e);
                Err(e)?
            }
        }
    }

    fn send_answer(&self, text: impl Into<String>, chat_id: u64) -> Result<()> {
        let text = text.into();
        log::debug!("Start to send answer! Text: {}, chat id: {}", text, chat_id);
        let answer_data = json!({
         "chat_id": chat_id,
         "text": text,
         "parse_mode": "MarkdownV2"
        });
        let url = format!("{}{}/sendMessage", TG_API_PREFIX, self.token);
        log::trace!("Requset url: {}", url);
        match self.http.post(url, answer_data.to_string()) {
            Ok(r) => {
                log::trace!("Send response: {:#?}", r);
            }
            Err(e) => {
                log::error!("Error: {}", e);
                Err(e)?
            }
        };
        Ok(())
    }
}

struct MessageProcessor {
    store: HashMap<u64, UserContext>,
    dispatcher: HandlerDispatcher,
}

impl MessageProcessor {
    fn new() -> Self {
        MessageProcessor {
            store: HashMap::new(),
            dispatcher: HandlerDispatcher::new(),
        }
    }

    fn process_message(&mut self, message: Message) -> Option<String> {
        log::debug!("Receive message: {:#?}", message);
        match message.text {
            Some(text) => {
                let user_id = message.from.id;
                let store = &mut self.store;
                let user_context = match store.remove(&user_id) {
                    Some(context) => context,
                    None => UserContext::new(user_id),
                };
                let answer = self.dispatcher.handle(text, user_context);
                if answer.1.user_state != State::Finished {
                    store.insert(user_id, answer.1);
                }
                Some(answer.0)
            }
            _ => {
                log::debug!("Text message is absent in update!");
                None
            }
        }
    }
}
