use std::{
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::Duration,
};

#[derive(Debug)]
pub struct Glob {
    users: Mutex<Vec<String>>,
}

impl Glob {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(Vec::new()),
        }
    }

    pub fn add_one(&self, user_id: String) {
        if user_id == "4".to_string() {
            sleep(Duration::from_secs(2));
        }
        let mut users = self.users.lock().unwrap();
        users.push(user_id);
    }
}

pub fn try_vec() {
    let glob = Glob::new();
    let this = Arc::new(glob);
    for l in 0..10 {
        let this = this.clone();
        spawn(move || {
            this.add_one(l.to_string());
            println!("{:?}", this.users);
        });
    }
}
