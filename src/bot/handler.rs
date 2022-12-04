use crate::domain::model::{CardioExersice, State, UserContext};

///Команды и тренировки оформить как-нить отдельно
const START: &'static str = "/START";
const STOP: &'static str = "/STOP";
const HELP: &'static str = "/HELP";
const RUNNING: &'static str = "/RUN";
const CYCLING: &'static str = "/CYCLING";
const TRAININGS: &'static [&'static str] = &[RUNNING, CYCLING];

pub trait Handler {
    fn handle(
        &self,
        message: impl Into<String>,
        user_context: UserContext,
    ) -> (String, UserContext);
}
//any
struct HelpHandler {}
//start
struct StartHandler {}
//exers
struct SelectExerHandler {}
//stop
struct StopTrainingHandler {}
//select distanse
struct SelectDistanceHandler {}

struct UnknownHandler {}

pub struct HandlerDispatcher {
    help: HelpHandler,
    start: StartHandler,
    select_exersice: SelectExerHandler,
    stop: StopTrainingHandler,
    select_distance: SelectDistanceHandler,
    unknown: UnknownHandler,
}

impl HandlerDispatcher {
    pub fn new() -> HandlerDispatcher {
        HandlerDispatcher {
            help: HelpHandler {},
            start: StartHandler {},
            select_exersice: SelectExerHandler {},
            stop: StopTrainingHandler {},
            select_distance: SelectDistanceHandler {},
            unknown: UnknownHandler {},
        }
    }
}

impl Handler for HelpHandler {
    fn handle(
        &self,
        _message: impl Into<String>,
        user_context: UserContext,
    ) -> (String, UserContext) {
        (
            format!("Выбери {} для начала тренировки", START),
            user_context,
        )
    }
}

impl Handler for StartHandler {
    fn handle(
        &self,
        _message: impl Into<String>,
        user_context: UserContext,
    ) -> (String, UserContext) {
        let mut context = user_context;
        context.user_state = State::ChooseTraining;
        let answer = format!("Выбери тренировку: {}, {}", RUNNING, CYCLING);
        (answer, context)
    }
}

impl Handler for SelectExerHandler {
    fn handle(
        &self,
        message: impl Into<String>,
        user_context: UserContext,
    ) -> (String, UserContext) {
        let message = message.into();
        if !TRAININGS.contains(&message.as_str()) {
            let all_trainings: String = TRAININGS.into_iter().map(|t| format!("[{}]", t)).collect();
            (
                format!("Не верное упражнение\\. Введите одно из: {}", all_trainings),
                user_context,
            )
        } else {
            let mut context = user_context;
            context.user_state = State::TrainingInProgres;
            message.chars().next();
            context
                .training
                .add_exersise(CardioExersice::start(message));
            (format!("Нажмите {} для завершения", STOP), context)
        }
    }
}

impl Handler for StopTrainingHandler {
    fn handle(
        &self,
        message: impl Into<String>,
        user_context: UserContext,
    ) -> (String, UserContext) {
        let message = message.into();
        if STOP != message {
            (
                format!("Для завершения тренировки нажмите: {}", STOP),
                user_context,
            )
        } else {
            let mut context = user_context;
            context.user_state = State::SelectDistance;
            ("Введите пройденную дистанцию".to_owned(), context)
        }
    }
}

impl Handler for SelectDistanceHandler {
    fn handle(
        &self,
        message: impl Into<String>,
        user_context: UserContext,
    ) -> (String, UserContext) {
        let distance = message.into();
        match distance.parse::<u32>() {
            Ok(d) => {
                let mut context = user_context;
                context.user_state = State::Finished;
                if let Some(exersice) = context.training.get_exersises().first_mut() {
                    exersice.finish(d);
                    return (
                        format!(
                            "Тренировка завершена\\. Пройденная дистанция: {}, время: {}",
                            exersice.distance,
                            exersice.training_time.as_secs()
                        ),
                        context,
                    );
                }
                ("Тренировка завершена".to_owned(), context)
            }
            Err(_) => (
                format!(
                    "Дистанция должна быть положительным числом\\. Вы ввели: {}",
                    distance
                ),
                user_context,
            ),
        }
    }
}

impl Handler for UnknownHandler {
    fn handle(
        &self,
        message: impl Into<String>,
        user_context: UserContext,
    ) -> (String, UserContext) {
        let message = message.into();
        (
            format!(
                "Неизвестная команда: {}, введите {} для получения помощи",
                message, HELP
            ),
            user_context,
        )
    }
}

impl Handler for HandlerDispatcher {
    fn handle(
        &self,
        message: impl Into<String>,
        user_context: UserContext,
    ) -> (String, UserContext) {
        let message = message.into();
        log::debug!(
            "[HandlerDispatcher] receive message: {} and user context: {:#?}",
            message,
            user_context
        );
        let state = user_context.user_state;
        let ansewr = if message == HELP {
            self.help.handle(message, user_context)
        } else if message == START && state == State::ReadyToStart {
            self.start.handle(message, user_context)
        } else if TRAININGS.contains(&message.as_str()) && state == State::ChooseTraining {
            self.select_exersice.handle(message, user_context)
        } else if message == STOP && state == State::TrainingInProgres {
            self.stop.handle(message, user_context)
        } else if state == State::SelectDistance {
            self.select_distance.handle(message, user_context)
        } else {
            self.unknown.handle(message, user_context)
        };
        log::debug!("[HandlerDispatcher] answer is: {:#?}", ansewr);
        ansewr
    }
}
