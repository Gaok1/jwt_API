use serde::{Deserialize, Serialize};

use crate::model::event::{Event, EventType};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsultEventRequest {
    event_name: String,
}

impl ConsultEventRequest {
    pub fn event_name(&self) -> &str {
        &self.event_name
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WholeEventRequest {//validar com o token
    event_name: String,
    description: String,
    event_type: EventType,
    date: String,
}

impl WholeEventRequest { 
    pub fn event_name(&self) -> &str {
        &self.event_name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn event_type(&self) -> EventType {
        self.event_type
    }
    pub fn date(&self) -> &str {
        &self.date
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteEventRequest { //validar com o token
    event_id: String,
}

impl DeleteEventRequest {
    pub fn event_id(&self) -> &str {
        &self.event_id
    }
}
