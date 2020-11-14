use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Write;

use crate::ruleset::Ruleset;

const LOGGING: bool = true;
const LOG_PATH: &'static str = "logs/";

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    id: String,
    ruleset: Ruleset,
    timeout: i32
}

impl Game {
    pub fn _new(id: String, ruleset: Ruleset, timeout: i32) -> Game {
        Game {id, ruleset, timeout}
    }

    pub fn _get_id(&self) -> &String {
        &self.id
    }

    pub fn _get_ruleset(&self) -> &Ruleset {
        &self.ruleset
    }

    pub fn _get_timeout(&self) -> i32 {
        self.timeout
    }

    pub fn log_data(&self, data: String) {
        println!("{}", data);
        if LOGGING{
    
            let mut file: File = OpenOptions::new()
                .append(true)
                .create(true)
                .open(format!("{}{}.log", LOG_PATH, self.id))
                .unwrap();
            
            file.write_all(data.as_bytes()).unwrap();
        }
    }
}
