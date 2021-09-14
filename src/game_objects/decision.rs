use crate::coordinate::Coordinate;

pub struct Decision {
    down_survival: bool,
    up_survival: bool,
    right_survival: bool,
    left_survival: bool,

    down_best: bool,
    up_best: bool,
    right_best: bool,
    left_best: bool,

    max_area: i32,
    down_area: i32,
    up_area: i32,
    right_area: i32,
    left_area: i32,

    can_escape_down: bool,
    can_escape_up: bool,
    can_escape_right: bool,
    can_escape_left: bool,

    against_wall_down: bool,
    against_wall_up: bool,
    against_wall_right: bool,
    against_wall_left: bool,

    food_down: bool,
    food_up: bool,
    food_right: bool,
    food_left: bool,

    food_exists: bool,
    closest_food: Coordinate,

    weak_snake_exists: bool,
    weak_snake_head: Coordinate,

    will_kill: bool,

    current_pos: Coordinate,
}

impl Decision {
    pub fn new(
        down_survival: bool,
        up_survival: bool,
        right_survival: bool,
        left_survival: bool,

        down_best: bool,
        up_best: bool,
        right_best: bool,
        left_best: bool,

        max_area: i32,
        down_area: i32,
        up_area: i32,
        right_area: i32,
        left_area: i32,

        can_escape_down: bool,
        can_escape_up: bool,
        can_escape_right: bool,
        can_escape_left: bool,

        against_wall_down: bool,
        against_wall_up: bool,
        against_wall_right: bool,
        against_wall_left: bool,

        food_down: bool,
        food_up: bool,
        food_right: bool,
        food_left: bool,

        food_exists: bool,
        closest_food: Coordinate,

        weak_snake_exists: bool,
        weak_snake_head: Coordinate,

        will_kill: bool,

        current_pos: Coordinate,
    ) -> Decision {
        Decision {
            down_survival,
            up_survival,
            right_survival,
            left_survival,

            down_best,
            up_best,
            right_best,
            left_best,

            max_area,
            down_area,
            up_area,
            right_area,
            left_area,

            can_escape_down,
            can_escape_up,
            can_escape_right,
            can_escape_left,

            against_wall_down,
            against_wall_up,
            against_wall_right,
            against_wall_left,

            food_down,
            food_up,
            food_right,
            food_left,

            food_exists,
            closest_food,

            weak_snake_exists,
            weak_snake_head,

            will_kill,

            current_pos,
        }
    }

    pub fn calculate(self) -> (i32, String) {
        // Take all data and decide on direction
        let outcome;
        let direction;

        // Move towards kill with best move
        if self.down_survival && self.will_kill && self.down_best {
            outcome = 0;
            direction = String::from("down");
        } else if self.up_survival && self.will_kill && self.up_best {
            outcome = 1;
            direction = String::from("up");
        } else if self.right_survival && self.will_kill && self.right_best {
            outcome = 2;
            direction = String::from("right");
        } else if self.left_survival && self.will_kill && self.left_best {
            outcome = 3;
            direction = String::from("left");
        }
        // Move towards closest weak snake with best move, avoiding walls
        else if self.down_survival
            && self.can_escape_down
            && self.down_best
            && (!self.against_wall_down || self.food_down)
            && self.weak_snake_exists
            && self.weak_snake_head.get_y() < self.current_pos.get_y()
        {
            outcome = 4;
            direction = String::from("down");
        } else if self.up_survival
            && self.can_escape_up
            && self.up_best
            && (!self.against_wall_up || self.food_up)
            && self.weak_snake_exists
            && self.weak_snake_head.get_y() > self.current_pos.get_y()
        {
            outcome = 5;
            direction = String::from("up");
        } else if self.right_survival
            && self.can_escape_right
            && self.right_best
            && (!self.against_wall_right || self.food_right)
            && self.weak_snake_exists
            && self.weak_snake_head.get_x() > self.current_pos.get_x()
        {
            outcome = 6;
            direction = String::from("right");
        } else if self.left_survival
            && self.can_escape_left
            && self.left_best
            && (!self.against_wall_left || self.food_left)
            && self.weak_snake_exists
            && self.weak_snake_head.get_x() < self.current_pos.get_x()
        {
            outcome = 7;
            direction = String::from("left");
        }
        // Move towards closest food with best move, avoiding walls
        else if self.down_survival
            && self.can_escape_down
            && self.down_best
            && (!self.against_wall_down || self.food_down)
            && self.food_exists
            && self.closest_food.get_y() < self.current_pos.get_y()
        {
            outcome = 8;
            direction = String::from("down");
        } else if self.up_survival
            && self.can_escape_up
            && self.up_best
            && (!self.against_wall_up || self.food_up)
            && self.food_exists
            && self.closest_food.get_y() > self.current_pos.get_y()
        {
            outcome = 9;
            direction = String::from("up");
        } else if self.right_survival
            && self.can_escape_right
            && self.right_best
            && (!self.against_wall_right || self.food_right)
            && self.food_exists
            && self.closest_food.get_x() > self.current_pos.get_x()
        {
            outcome = 10;
            direction = String::from("right");
        } else if self.left_survival
            && self.can_escape_left
            && self.left_best
            && (!self.against_wall_left || self.food_left)
            && self.food_exists
            && self.closest_food.get_x() < self.current_pos.get_x()
        {
            outcome = 11;
            direction = String::from("left");
        }
        // Move towards escape with best move, avoiding walls
        else if self.down_survival
            && self.can_escape_down
            && self.down_best
            && (!self.against_wall_down || self.food_down)
        {
            outcome = 12;
            direction = String::from("down");
        } else if self.up_survival
            && self.can_escape_up
            && self.up_best
            && (!self.against_wall_up || self.food_up)
        {
            outcome = 13;
            direction = String::from("up");
        } else if self.right_survival
            && self.can_escape_right
            && self.right_best
            && (!self.against_wall_right || self.food_right)
        {
            outcome = 14;
            direction = String::from("right");
        } else if self.left_survival
            && self.can_escape_left
            && self.left_best
            && (!self.against_wall_left || self.food_left)
        {
            outcome = 15;
            direction = String::from("left");
        }
        // Move towards closest weak snake with best move
        else if self.down_survival
            && self.can_escape_down
            && self.down_best
            && self.weak_snake_exists
            && self.weak_snake_head.get_y() < self.current_pos.get_y()
        {
            outcome = 16;
            direction = String::from("down");
        } else if self.up_survival
            && self.can_escape_up
            && self.up_best
            && self.weak_snake_exists
            && self.weak_snake_head.get_y() > self.current_pos.get_y()
        {
            outcome = 17;
            direction = String::from("up");
        } else if self.right_survival
            && self.can_escape_right
            && self.right_best
            && self.weak_snake_exists
            && self.weak_snake_head.get_x() > self.current_pos.get_x()
        {
            outcome = 18;
            direction = String::from("right");
        } else if self.left_survival
            && self.can_escape_left
            && self.left_best
            && self.weak_snake_exists
            && self.weak_snake_head.get_x() < self.current_pos.get_x()
        {
            outcome = 19;
            direction = String::from("left");
        }
        // Move towards closest food with best move
        else if self.down_survival
            && self.can_escape_down
            && self.down_best
            && self.food_exists
            && self.closest_food.get_y() < self.current_pos.get_y()
        {
            outcome = 20;
            direction = String::from("down");
        } else if self.up_survival
            && self.can_escape_up
            && self.up_best
            && self.food_exists
            && self.closest_food.get_y() > self.current_pos.get_y()
        {
            outcome = 21;
            direction = String::from("up");
        } else if self.right_survival
            && self.can_escape_right
            && self.right_best
            && self.food_exists
            && self.closest_food.get_x() > self.current_pos.get_x()
        {
            outcome = 22;
            direction = String::from("right");
        } else if self.left_survival
            && self.can_escape_left
            && self.left_best
            && self.food_exists
            && self.closest_food.get_x() < self.current_pos.get_x()
        {
            outcome = 23;
            direction = String::from("left");
        }
        // Move towards escape with best move
        else if self.down_survival && self.can_escape_down && self.down_best {
            outcome = 24;
            direction = String::from("down");
        } else if self.up_survival && self.can_escape_up && self.up_best {
            outcome = 25;
            direction = String::from("up");
        } else if self.right_survival && self.can_escape_right && self.right_best {
            outcome = 16;
            direction = String::from("right");
        } else if self.left_survival && self.can_escape_left && self.left_best {
            outcome = 27;
            direction = String::from("left");
        }
        // Move towards escape
        else if self.down_survival && self.can_escape_down {
            outcome = 28;
            direction = String::from("down");
        } else if self.up_survival && self.can_escape_up {
            outcome = 29;
            direction = String::from("up");
        } else if self.right_survival && self.can_escape_right {
            outcome = 30;
            direction = String::from("right");
        } else if self.left_survival && self.can_escape_left {
            outcome = 31;
            direction = String::from("left");
        }
        // Move towards best move, can escape
        else if self.can_escape_down && self.down_best {
            outcome = 32;
            direction = String::from("down");
        } else if self.can_escape_up && self.up_best {
            outcome = 33;
            direction = String::from("up");
        } else if self.can_escape_right && self.right_best {
            outcome = 34;
            direction = String::from("right");
        } else if self.can_escape_left && self.left_best {
            outcome = 35;
            direction = String::from("left");
        }
        // Move towards escape
        else if self.can_escape_down {
            outcome = 36;
            direction = String::from("down");
        } else if self.can_escape_up {
            outcome = 37;
            direction = String::from("up");
        } else if self.can_escape_right {
            outcome = 38;
            direction = String::from("right");
        } else if self.can_escape_left {
            outcome = 39;
            direction = String::from("left");
        }
        // Go for best move with most turns, no survival
        else if self.down_best && self.down_area == self.max_area {
            outcome = 40;
            direction = String::from("down");
        } else if self.up_best && self.up_area == self.max_area {
            outcome = 41;
            direction = String::from("up");
        } else if self.right_best && self.right_area == self.max_area {
            outcome = 42;
            direction = String::from("right");
        } else if self.left_best && self.left_area == self.max_area {
            outcome = 43;
            direction = String::from("left")
        }
        // Go for most turns, no survival
        else if self.down_area == self.max_area {
            outcome = 44;
            direction = String::from("down");
        } else if self.up_area == self.max_area {
            outcome = 45;
            direction = String::from("up");
        } else if self.right_area == self.max_area {
            outcome = 46;
            direction = String::from("right");
        } else if self.left_area == self.max_area {
            outcome = 47;
            direction = String::from("left")
        }
        // Default
        else {
            outcome = 48;
            direction = String::from("up");
        }

        (outcome, direction)
    }
}
