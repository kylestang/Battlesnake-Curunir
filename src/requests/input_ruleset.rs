use crate::ruleset::Ruleset;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InputRuleset {
    name: String,
    version: String,
    settings: InputSettings,
}

impl InputRuleset {
    pub fn into_ruleset(self) -> Ruleset {
        let (
            food_spawn_chance,
            minimum_food,
            hazard_damage_per_turn,
            shrink_every_n_turns,
            allowed_body_collisions,
            shared_elimination,
            shared_health,
            shared_length,
        ) = self.settings.into_values();

        Ruleset::new(
            self.name,
            self.version,
            food_spawn_chance,
            minimum_food,
            hazard_damage_per_turn,
            shrink_every_n_turns,
            allowed_body_collisions,
            shared_elimination,
            shared_health,
            shared_length,
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InputSettings {
    foodSpawnChance: i32,
    minimumFood: i32,
    hazardDamagePerTurn: i32,
    royale: InputRoyale,
    squad: InputSquad,
}

impl InputSettings {
    pub fn into_values(self) -> (i32, i32, i32, i32, bool, bool, bool, bool) {
        let shrink_every_n_turns = self.royale.into_values();
        let (allowed_body_collisions, shared_elimination, shared_health, shared_length) =
            self.squad.into_values();

        (
            self.foodSpawnChance,
            self.minimumFood,
            self.hazardDamagePerTurn,
            shrink_every_n_turns,
            allowed_body_collisions,
            shared_elimination,
            shared_health,
            shared_length,
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InputRoyale {
    shrinkEveryNTurns: i32,
}

impl InputRoyale {
    pub fn into_values(self) -> i32 {
        self.shrinkEveryNTurns
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InputSquad {
    allowBodyCollisions: bool,
    sharedElimination: bool,
    sharedHealth: bool,
    sharedLength: bool,
}

impl InputSquad {
    pub fn into_values(self) -> (bool, bool, bool, bool) {
        (
            self.allowBodyCollisions,
            self.sharedElimination,
            self.sharedHealth,
            self.sharedLength,
        )
    }
}
