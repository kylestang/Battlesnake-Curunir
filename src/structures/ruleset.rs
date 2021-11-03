use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Ruleset {
    name: String,
    version: String,
    food_spawn_chance: i32,
    minimum_food: i32,
    hazard_damage_per_turn: i32,
    shrink_every_n_turns: i32,
    allow_body_collisions: bool,
    shared_elimination: bool,
    shared_health: bool,
    shared_length: bool,
}

impl Ruleset {
    pub fn new(
        name: String,
        version: String,
        food_spawn_chance: i32,
        minimum_food: i32,
        hazard_damage_per_turn: i32,
        shrink_every_n_turns: i32,
        allow_body_collisions: bool,
        shared_elimination: bool,
        shared_health: bool,
        shared_length: bool,
    ) -> Ruleset {
        Ruleset {
            name,
            version,
            food_spawn_chance,
            minimum_food,
            hazard_damage_per_turn,
            shrink_every_n_turns,
            allow_body_collisions,
            shared_elimination,
            shared_health,
            shared_length,
        }
    }

    pub fn _get_name(&self) -> &String {
        &self.name
    }

    pub fn _get_version(&self) -> &String {
        &self.version
    }

    pub fn get_minimum_food(&self) -> i32 {
        self.minimum_food
    }
}
