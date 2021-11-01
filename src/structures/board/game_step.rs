use crate::board::Board;
use crate::ruleset::Ruleset;

impl Board {
    pub fn game_step(&mut self, ruleset: &Ruleset) {
        // Check all food
        let mut i = 0;

        while i < self.food.len() {
            let mut food_eaten = false;

            // Check all snakes
            for j in 0..self.snakes.len() {
                if self.snakes[j].get_head() == self.food[i] {
                    food_eaten = true;
                    self.snakes[j].eat_food();
                }
            }

            // Remove food if eaten
            if food_eaten {
                self.food.swap_remove(i);
            } else {
                i += 1;
            }
        }

        /*
        Any Battlesnake that has been eliminated is removed from the game board:
            Health less than or equal to 0
            Moved out of bounds
            Collided with themselves
            Collided with another Battlesnake
            Collided head-to-head and lost
        */

        // Eliminate snakes that are out of health or out of bounds
        let mut i = 0;

        while i < self.snakes.len() {
            let snake = &self.snakes[i];
            let x = snake.get_head().get_x();
            let y = snake.get_head().get_y();

            if snake.get_health() < ruleset.get_minimum_food()
                || (x < 0 || x > self.width - 1 || y < 0 || y > self.height - 1)
            {
                self.snakes.remove(i);
            } else {
                i += 1;
            }
        }

        // Check for collisions
        let mut to_remove = Vec::with_capacity(self.snakes.len());
        for snake in &self.snakes {
            for other_snake in &self.snakes {
                if snake.lost_headon(other_snake) || snake.body_collision_with(other_snake) {
                    to_remove.push(snake.get_id());
                    break;
                }
            }
        }

        self.get_snakes_mut()
            .retain(|snake| !to_remove.contains(&snake.get_id()));

        self.increment_turn();
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::_TEST_PATH;
    use crate::load_object;

    // game_step()
    #[test]
    fn test_body_collision() {
        let mut before_collision = load_object!(Board, "body_collision-01-before", _TEST_PATH);
        let mut after_collision = load_object!(Board, "body_collision-01-after", _TEST_PATH);
        let ruleset = load_object!(Ruleset, "body_collision-01-before", _TEST_PATH);
        after_collision.set_max_snakes(2);

        before_collision.game_step(&ruleset);

        assert_eq!(before_collision, after_collision);
    }

    #[test]
    fn test_double_headon_collision() {
        let mut before_collision =
            load_object!(Board, "double_headon_collision-01-before", _TEST_PATH);
        let mut after_collision =
            load_object!(Board, "double_headon_collision-01-after", _TEST_PATH);
        let ruleset = load_object!(Ruleset, "double_headon_collision-01-before", _TEST_PATH);
        after_collision.set_max_snakes(3);

        before_collision.game_step(&ruleset);

        assert_eq!(before_collision, after_collision);
    }

    #[test]
    fn test_eat_food() {
        let mut before_eat = load_object!(Board, "eat-01-before", _TEST_PATH);
        let after_eat = load_object!(Board, "eat-01-after", _TEST_PATH);
        let ruleset = load_object!(Ruleset, "eat-01-before", _TEST_PATH);

        before_eat.game_step(&ruleset);

        assert_eq!(before_eat, after_eat);
    }

    #[test]
    fn test_headon_collision() {
        let mut before_collision = load_object!(Board, "headon_collision-01-before", _TEST_PATH);
        let mut after_collision = load_object!(Board, "headon_collision-01-after", _TEST_PATH);
        let ruleset = load_object!(Ruleset, "headon_collision-01-before", _TEST_PATH);
        after_collision.set_max_snakes(2);

        before_collision.game_step(&ruleset);

        assert_eq!(before_collision, after_collision);
    }

    #[test]
    fn test_out_of_bounds() {
        let mut before = load_object!(Board, "out_of_bounds-01-before", _TEST_PATH);
        let mut after = load_object!(Board, "out_of_bounds-01-after", _TEST_PATH);
        let ruleset = load_object!(Ruleset, "out_of_bounds-01-before", _TEST_PATH);
        after.set_max_snakes(2);

        before.game_step(&ruleset);

        assert_eq!(before, after);
    }

    #[test]
    fn test_out_of_health() {
        let mut before = load_object!(Board, "out_of_health-01-before", _TEST_PATH);
        let mut after = load_object!(Board, "out_of_health-01-after", _TEST_PATH);
        let ruleset = load_object!(Ruleset, "out_of_health-01-before", _TEST_PATH);
        after.set_max_snakes(2);

        before.game_step(&ruleset);

        assert_eq!(before, after);
    }

    #[test]
    fn test_no_change() {
        let mut before = load_object!(Board, "no_change-01-before", _TEST_PATH);
        let after = load_object!(Board, "no_change-01-after", _TEST_PATH);
        let ruleset = load_object!(Ruleset, "no_change-01-before", _TEST_PATH);

        before.game_step(&ruleset);

        assert_eq!(before, after);
    }
}
