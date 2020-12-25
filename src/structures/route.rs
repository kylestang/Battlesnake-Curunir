use std::cmp::Ordering;

pub const MAX_ROUTE: Route = Route{survival: true, turns: i32::MAX, solo: true, snakes_killed: i32::MAX, my_food: i32::MAX, opponent_food: 0};
pub const MIN_ROUTE: Route = Route{survival: false, turns: 0, solo: false, snakes_killed: 0, my_food: 0, opponent_food: i32::MAX};
pub const DEFAULT_ROUTE: Route = Route{survival: true, turns: 0, solo: false, snakes_killed: 0, my_food: 0, opponent_food: 0};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Route {
    survival: bool,
    turns: i32,
    solo: bool,
    snakes_killed: i32,
    my_food: i32,
    opponent_food: i32
}

impl Route {
    pub fn _new(survival: bool, turns: i32, solo: bool, snakes_killed: i32, my_food: i32, opponent_food: i32) -> Route {
        Route {survival, turns, solo, snakes_killed, my_food, opponent_food}
    }

    pub fn get_survival(&self) -> bool {
        self.survival
    }

    pub fn set_survival(&mut self, survival: bool) {
        self.survival = survival;
    }

    pub fn increment_turns(&mut self) {
        self.turns += 1;
    }

    pub fn set_solo(&mut self, solo:bool) {
        self.solo = solo;
    }

    pub fn increment_snakes_killed(&mut self) {
        self.snakes_killed += 1;
    }

    pub fn increment_my_food(&mut self) {
        self.my_food += 1;
    }

    pub fn increment_opponent_food(&mut self) {
        self.opponent_food += 1;
    }
}

impl PartialOrd for Route {
    fn partial_cmp(&self, other: &Route) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Route {
    fn cmp(&self, other: &Route) -> Ordering {
        // Route where self survives is greater
        if self.survival && !other.survival {
            return Ordering::Greater;
        } else if !self.survival && other.survival {
            return Ordering::Less;
        } 

        // Route where self survives more turns is greater
        else if self.turns > other.turns {
            return Ordering::Greater;
        } else if self.turns < other.turns {
            return Ordering::Less;
        }

        // Route where self kills more snakes is greater
        else if self.snakes_killed > other.snakes_killed {
            return Ordering::Greater;
        } else if self.snakes_killed < other.snakes_killed {
            return Ordering::Less;
        }

        // Route where self eats more food is greater
        else if self.my_food > other.my_food {
            return Ordering::Greater;
        } else if self.my_food < other.my_food {
            return Ordering::Less;
        }

        // Route where opponents eat less food is greater
        else if self.opponent_food < other.opponent_food {
            return Ordering::Greater;
        } else if self.opponent_food > other.opponent_food {
            return Ordering::Less;
        }

        // Route where self is solo is greater
        else if self.solo && !other.solo {
            return Ordering::Greater;
        } else if !self.solo && other.solo {
            return Ordering::Less;
        }

        // All variables are equal
        else {
            return Ordering::Equal;
        }
    }
}
