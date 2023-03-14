use std::{
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::Duration,
};

#[derive(Debug)]
pub struct Glob {
    users: Vec<String>,
}

impl Glob {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }

    pub fn add_one(&mut self, user_id: String) {
        if user_id == "4".to_string() {
            sleep(Duration::from_secs(4));
        }
        self.users.push(user_id);
    }
}

pub fn try_vec() {
    let glob = Glob::new();
    let this = Arc::new(Mutex::new(glob));
    for l in 0..10 {
        let this = this.clone();
        spawn(move || {
            let mut this = this.lock().unwrap();
            this.add_one(l.to_string());
            this.add_one((l + 1).to_string());
            println!("{:?}", &this.users);
        });
    }
}
