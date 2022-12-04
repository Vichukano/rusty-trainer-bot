use std::fmt::Debug;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Response result is FALSE!")]
    BadResponseResultError,
    #[error("Wrong status code")]
    BadStatusCode(u16),
}
