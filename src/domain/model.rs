use chrono::{DateTime, Local};

use super::user_state::State;

pub struct UserContext {
    pub user_id: u64,
    pub user_stat: State,
    pub training: Training,
}

pub enum Training {
    Cardio(CardioTraining),
    Gym(GymTraining),
}

pub struct CardioTraining {
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
    pub exersices: Vec<CardioExersice>,
}

pub struct CardioExersice {
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
    name: String,
    distance: u32,
}

pub struct GymTraining {
    start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    pub exersices: Vec<GymExersice>,
}

pub struct GymExersice {
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
    name: String,
    weight: u32,
    reps: u32,
}
