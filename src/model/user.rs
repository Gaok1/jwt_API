use std::sync::atomic::AtomicI32;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct User {
    id: i32,
    name: String,
    password: String,
}

impl User {
    pub fn new_with_id(id: i32, name: String, password: String) -> User {
        User { id, name, password }
    }
    pub fn new(name: String, password: String) -> User {
        let id = last_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        User { id, name, password }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}
static last_id: AtomicI32 = AtomicI32::new(0);
