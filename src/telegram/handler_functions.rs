use crate::{domain::user_state::State, telegram::messages::HELP_MESSAGE};

#[derive(Debug)]
pub struct MessageContext {
    user_id: u64,
    message_text: String,
    state: State,
}

#[derive(Debug)]
pub struct AnswerContext {
    user_id: u64,
    answer: String,
    state: State,
}

pub fn help(context: MessageContext) -> AnswerContext {
    log::debug!("Start to handle context: {:#?}", context);
    AnswerContext {
        user_id: context.user_id,
        answer: HELP_MESSAGE.to_owned(),
        state: context.state,
    }
}

pub fn start(context: MessageContext) -> AnswerContext {
    log::debug!("Start to handle context: {:#?}", context);
    AnswerContext {
        user_id: 0,
        answer: "!!".to_owned(),
        state: State::START_STATE,
    }
}

pub fn cardio(context: MessageContext) -> AnswerContext {
    log::debug!("Start to handle context: {:#?}", context);
    AnswerContext {
        user_id: 0,
        answer: "".to_owned(),
        state: State::START_STATE,
    }
}

pub fn stop_cardio(context: MessageContext) -> AnswerContext {
    log::debug!("Start to handle context: {:#?}", context);
    AnswerContext {
        user_id: 0,
        answer: "".to_owned(),
        state: State::START_STATE,
    }
}

pub fn cardio_input_distance(context: MessageContext) -> AnswerContext {
    log::debug!("Start to handle context: {:#?}", context);
    AnswerContext {
        user_id: 0,
        answer: "".to_owned(),
        state: State::START_STATE,
    }
}
