use std::{time::Duration, time::Instant};

#[derive(Debug)]
pub struct UserContext {
    pub user_id: u64,
    pub user_state: State,
    pub training: CardioTraining,
}

impl UserContext {
    pub fn new(id: u64) -> Self {
        UserContext {
            user_id: id,
            user_state: State::ReadyToStart,
            training: CardioTraining::start(),
        }
    }
}

#[derive(Debug)]
pub struct CardioTraining {
    start_time: Instant,
    end_time: Duration,
    exersices: Vec<CardioExersice>,
}

impl CardioTraining {
    pub fn start() -> Self {
        CardioTraining {
            start_time: Instant::now(),
            end_time: Duration::ZERO,
            exersices: Vec::new(),
        }
    }

    pub fn add_exersise(&mut self, exersice: CardioExersice) {
        self.exersices.push(exersice)
    }

    pub fn get_exersises(&mut self) -> &mut Vec<CardioExersice> {
        &mut self.exersices
    }
}

#[derive(Debug)]
pub struct CardioExersice {
    start_time: Instant,
    name: String,
    pub training_time: Duration,
    pub distance: u32,
}

impl CardioExersice {
    pub fn start(name: String) -> Self {
        CardioExersice {
            start_time: Instant::now(),
            name,
            training_time: Duration::ZERO,
            distance: 0,
        }
    }

    pub fn finish(&mut self, distanse: u32) {
        self.training_time = self.start_time.elapsed();
        self.distance = distanse;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    ChooseTraining,
    ReadyToStart,
    TrainingInProgres,
    SelectDistance,
    Finished,
}
