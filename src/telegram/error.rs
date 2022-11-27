use std::fmt::{Debug, Display};

pub enum BotError {
    BadResponseResultError,
    RequestExecutionError(ureq::Error),
    ParseResponseError(std::io::Error),
    BadStatusCode(u16),
}

impl BotError {
    fn to_string(&self) -> String {
        match self {
            BotError::BadResponseResultError => "Receive bad response result!".to_owned(),
            BotError::RequestExecutionError(e) => {
                format!("Failed to send request to bot! Cause: {}", e)
            }
            BotError::ParseResponseError(e) => {
                format!("Failed to parce response from bot! Cause: {}", e)
            }
            BotError::BadStatusCode(code) => format!("Receive bad status code: {}", code),
        }
    }
}

impl Debug for BotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = self.to_string();
        write!(f, "{text}")
    }
}

impl Display for BotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = self.to_owned();
        write!(f, "{text}")
    }
}
