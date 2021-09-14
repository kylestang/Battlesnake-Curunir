use crate::game::Game;
use crate::input_ruleset::InputRuleset;
use crate::ruleset::Ruleset;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InputGame {
    id: String,
    ruleset: InputRuleset,
    timeout: i32,
}

impl InputGame {
    pub fn into_game(self) -> Game {
        Game::new(self.id, self.ruleset.into_ruleset(), self.timeout)
    }

    pub fn into_ruleset(self) -> Ruleset {
        self.ruleset.into_ruleset()
    }
}
