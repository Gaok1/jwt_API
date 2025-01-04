use std::{collections::HashMap, hash::Hash, sync::{LazyLock, Mutex}};

use crate::model::{event::Event, user::{self, User}};

pub enum StorageCell {
    User(User),
    Event(Event),
}

pub struct Database {
    storage: HashMap<i32, StorageCell>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }
    /// store a user in the database and return his id
    /// 
    /// ID are made by incrementing a global counter, `unique`
    pub fn insert_user(&mut self, user: User) -> i32 {
        let id = user.id();
        self.storage.insert(user.id(), StorageCell::User(user));
        id
    }

    pub fn insert_event(&mut self, event: Event) {
        self.storage.insert(event.id(), StorageCell::Event(event));
    }

    pub fn get_user(&self, id: i32) -> Option<&User> {
        match self.storage.get(&id) {
            Some(StorageCell::User(user)) => Some(user),
            _ => None,
        }
    }
    pub fn get_user_by_username(&self, username: &str) -> Option<&User> {
        self.storage
            .values()
            .find_map(|cell| match cell {
                StorageCell::User(user) if user.name() == username => Some(user),
                _ => None,
            })
    }

    pub fn get_event_by_id(&self, event_id: i32) -> Option<&Event> {
        match self.storage.get(&event_id) {
            Some(StorageCell::Event(event)) => Some(event),
            _ => None,
        }
    }
    pub fn get_event_by_name_id(&self, name: &str, user_id:i32) -> Option<Vec<&Event>> {
        let mut events = Vec::new();
        for cell in self.storage.values() {
            match cell {
                StorageCell::Event(event) if event.name() == name && event.user_id() == user_id => events.push(event),
                _ => (),
            }
        }
        if events.is_empty() {
            None
        } else {
            Some(events)
        }
    }

    pub fn delete_event_by_id(&mut self, id: i32) -> Result<Event, ()> {
        self.storage.remove(&id).map(|cell| match cell {
            StorageCell::Event(event) => event,
            _ => unreachable!(), //cala a boca compilador
        }).ok_or(())
    }
}

pub static DATABASE: LazyLock<Mutex<Database>> = LazyLock::new(|| Mutex::new(Database::new()));
