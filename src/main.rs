mod domain;
mod telegram;
use std::{env, thread, time::Duration};

use telegram::service::TelegramBotService;

fn main() {
    let env_load_result = dotenv::dotenv().ok();
    env_logger::init();
    log::info!("Result of dotenv: {}", env_load_result.is_some());
    let token = env::var("BOT_TOKEN").unwrap();
    let poll_timeout: u64 = env::var("POLL_DURATION_SECONDS").unwrap().parse().unwrap();
    log::trace!(
        "Starting bot! Token: {}, poll timeout: {}",
        token,
        poll_timeout
    );
    log::debug!("Starting bot! Poll timeout: {}", poll_timeout);
    let mut service = TelegramBotService::new(token);
    let mut offset = 0;
    loop {
        offset = service.handle_updates(offset).unwrap();
        thread::sleep(Duration::from_secs(poll_timeout));
    }
}
