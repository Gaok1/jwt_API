use serde::{Deserialize, Serialize};

use crate::model::event::Event;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsultEventRequest {
    user_id :i32,
    event_name : String,
    token : String,
}

impl ConsultEventRequest {
    pub fn user_id(&self) -> i32 {
        self.user_id
    }
    pub fn event_name(&self) -> &str {
        &self.event_name
    }
    pub fn token(&self) -> &str {
        &self.token
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WholeEventRequest {
    event : Event,
    token : String,
}

impl WholeEventRequest {
    pub fn event(&self) -> &Event {
        &self.event
    }
    pub fn token(&self) -> &str {
        &self.token
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteEventRequest {
    user_id :i32,
    event_id : String,
    token : String,
}

impl DeleteEventRequest {
    pub fn user_id(&self) -> i32 {
        self.user_id
    }
    pub fn event_id(&self) -> &str {
        &self.event_id
    }
    pub fn token(&self) -> &str {
        &self.token
    }
}