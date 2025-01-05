use std::sync::atomic::AtomicI32;

use serde::{Deserialize, Serialize};

use super::user::User;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event{
    id : i32,
    name : String,
    description : String,
    event_type : EventType,
    date : String, 
    user_id : i32,
}


#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(rename_all = "lowercase")] // se quiser serializar em minúsculas
pub enum EventType{
    Party,
    Meeting,
    Conference,
    Obligation,
}

impl Event {
    // Cria um evento simples; poderíamos usar um ID auto-increment ou UUID
    pub fn new_with_id(id: i32, name: String, description: String, event_type: EventType, date: String, user_id: i32) -> Self {
        Self { id, name, description, event_type, date, user_id }
    }
    pub fn new_event(name: String, description: String, event_type: EventType, date: String, user_id: i32) -> Self {
        let id = last_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Self { id, name, description, event_type, date, user_id }
    }

    pub fn generate_id(&mut self) {
        self.id = last_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn event_type(&self) -> &EventType {
        &self.event_type
    }
    pub fn user_id(&self) -> i32 {
        self.user_id
    }
}

static last_id: AtomicI32 = AtomicI32::new(0);
