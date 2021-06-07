use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Ruleset {
    name: String,
    version: String,
}

impl Ruleset {
    pub fn _new(name: String, version: String) -> Ruleset {
        Ruleset { name, version }
    }

    pub fn _get_name(&self) -> &String {
        &self.name
    }

    pub fn _get_version(&self) -> &String {
        &self.version
    }
}
