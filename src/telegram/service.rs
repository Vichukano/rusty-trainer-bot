use super::error::BotError;
use crate::domain::model::GetUpdateResponse;
use reqwest::blocking::Client;
use serde_json::json;

const TG_API_PREFIX: &str = "https://api.telegram.org/bot";

pub struct TelegramBotService {
    client: Client,
    token: String,
}

impl TelegramBotService {
    pub fn new(token: String) -> Self {
        TelegramBotService {
            client: Client::new(),
            token,
        }
    }

    pub fn handle_updates(&self, offset: u64) -> Result<u64, BotError> {
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
            if let Some(message) = update.message {
                log::debug!("Receive message: {:#?}", message);
                if let Some(text) = message.text {
                    log::trace!("Receive message text: {}", text);
                    let user_name = message.from.first_name;
                    let answer = format!("{} you send {}", user_name, text);
                    self.send_answer(answer.as_str(), message.chat.id)?;
                }
            }
        }
        Ok(max_update_id + 1)
    }

    fn get_updates(&self, offset: u64) -> Result<GetUpdateResponse, BotError> {
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

    fn send_answer(&self, text: &str, chat_id: u64) -> Result<(), BotError> {
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
