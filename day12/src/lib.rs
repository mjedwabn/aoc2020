pub mod day12 {
    use std::io::BufRead;

    pub fn navigate(input: &mut dyn BufRead) -> usize {
        let actions: Vec<(char, i32)> = parse_actions(input);
        let mut ship = Ship::new();
        ship.navigate(actions);
        return ship.get_distance_from_starting_location();
    }

    pub fn navigate_with_waypoint(input: &mut dyn BufRead) -> usize {
        let actions: Vec<(char, i32)> = parse_actions(input);
        let mut ship = ShipWithWaypoint::new();
        ship.navigate(actions);
        return ship.get_distance_from_starting_location();
    }

    pub struct Ship {
        starting_location: (i32, i32),
        location: (i32, i32),
        facing: i32,
    }

    pub struct ShipWithWaypoint {
        starting_location: (i32, i32),
        location: (i32, i32),
        waypoint_location: (i32, i32),
    }

    pub trait Locator {
        fn get_location(&self) -> (i32, i32);
        fn get_starting_location(&self) -> (i32, i32);
    }

    pub trait Navigator: Locator {
        fn navigate(&mut self, actions: Vec<(char, i32)>) {
            for (action, value) in actions {
                self.interpret_action(action, value);
            }
        }

        fn interpret_action(&mut self, action: char, value: i32);

        fn get_distance_from_starting_location(&self) -> usize {
            return ((self.get_location().0 - self.get_starting_location().0).abs()
                + (self.get_location().1 - self.get_starting_location().1).abs())
                as usize;
        }
    }

    impl Locator for Ship {
        fn get_location(&self) -> (i32, i32) {
            return self.location;
        }

        fn get_starting_location(&self) -> (i32, i32) {
            return self.starting_location;
        }
    }

    impl Locator for ShipWithWaypoint {
        fn get_location(&self) -> (i32, i32) {
            return self.location;
        }

        fn get_starting_location(&self) -> (i32, i32) {
            return self.starting_location;
        }
    }

    impl Navigator for Ship {
        fn interpret_action(&mut self, action: char, value: i32) {
            if action == 'N' {
                self.move_north(value);
            } else if action == 'S' {
                self.move_south(value);
            } else if action == 'E' {
                self.move_east(value);
            } else if action == 'W' {
                self.move_west(value);
            } else if action == 'L' {
                self.turn_left(value);
            } else if action == 'R' {
                self.turn_right(value);
            } else if action == 'F' {
                self.move_forward(value);
            }
        }
    }

    impl Ship {
        pub fn new() -> Ship {
            Ship {
                starting_location: (0, 0),
                location: (0, 0),
                facing: 90,
            }
        }

        pub fn get_facing(&self) -> i32 {
            return self.facing;
        }

        fn move_north(&mut self, distance: i32) {
            self.location = (self.location.0, self.location.1 + distance);
        }

        fn move_south(&mut self, distance: i32) {
            self.location = (self.location.0, self.location.1 - distance);
        }

        fn move_east(&mut self, distance: i32) {
            self.location = (self.location.0 + distance, self.location.1);
        }

        fn move_west(&mut self, distance: i32) {
            self.location = (self.location.0 - distance, self.location.1);
        }

        fn turn_left(&mut self, angle: i32) {
            self.facing = self.normalize_angle(self.facing - angle);
        }

        fn turn_right(&mut self, angle: i32) {
            self.facing = self.normalize_angle(self.facing + angle);
        }

        fn normalize_angle(&self, angle: i32) -> i32 {
            let a = angle % 360;
            return if a < 0 { 360 - a.abs() } else { a };
        }

        fn move_forward(&mut self, distance: i32) {
            if self.facing == 0 {
                self.move_north(distance);
            } else if self.facing == 90 {
                self.move_east(distance);
            } else if self.facing == 180 {
                self.move_south(distance);
            } else if self.facing == 270 {
                self.move_west(distance);
            }
        }
    }

    impl Navigator for ShipWithWaypoint {
        fn interpret_action(&mut self, action: char, value: i32) {
            if action == 'N' {
                self.move_waypoint_north(value);
            } else if action == 'S' {
                self.move_waypoint_south(value);
            } else if action == 'E' {
                self.move_waypoint_east(value);
            } else if action == 'W' {
                self.move_waypoint_west(value);
            } else if action == 'L' {
                self.rotate_waypoint_left(value);
            } else if action == 'R' {
                self.rotate_waypoint_right(value);
            } else if action == 'F' {
                self.move_forward_to_waypoint(value);
            }
        }
    }

    impl ShipWithWaypoint {
        pub fn new() -> ShipWithWaypoint {
            ShipWithWaypoint {
                starting_location: (0, 0),
                location: (0, 0),
                waypoint_location: (10, 1),
            }
        }

        fn move_waypoint_north(&mut self, distance: i32) {
            self.waypoint_location = (
                self.waypoint_location.0,
                self.waypoint_location.1 + distance,
            );
        }

        fn move_waypoint_south(&mut self, distance: i32) {
            self.waypoint_location = (
                self.waypoint_location.0,
                self.waypoint_location.1 - distance,
            );
        }

        fn move_waypoint_east(&mut self, distance: i32) {
            self.waypoint_location = (
                self.waypoint_location.0 + distance,
                self.waypoint_location.1,
            );
        }

        fn move_waypoint_west(&mut self, distance: i32) {
            self.waypoint_location = (
                self.waypoint_location.0 - distance,
                self.waypoint_location.1,
            );
        }

        fn rotate_waypoint_left(&mut self, angle: i32) {
            self.rotate_waypoint(self.normalize_angle(360 - angle));
        }

        fn rotate_waypoint_right(&mut self, angle: i32) {
            self.rotate_waypoint(self.normalize_angle(angle));
        }

        fn normalize_angle(&self, angle: i32) -> i32 {
            let a = angle % 360;
            return if a < 0 { 360 - a.abs() } else { a };
        }

        fn rotate_waypoint(&mut self, angle: i32) {
            let wp = self.get_relative_waypoint_position();
            let quarter_rotations = angle / 90;
            let mut new_rel_wp = (wp.0, wp.1);
            for _r in 0..quarter_rotations {
                new_rel_wp = (new_rel_wp.1, -new_rel_wp.0);
            }
            self.waypoint_location = (
                self.location.0 + new_rel_wp.0,
                self.location.1 + new_rel_wp.1,
            );
        }

        fn move_forward_to_waypoint(&mut self, times: i32) {
            let wp = self.get_relative_waypoint_position();
            let x_distance = wp.0 * times;
            let y_distance = wp.1 * times;
            self.location = (self.location.0 + x_distance, self.location.1 + y_distance);
            self.waypoint_location = (
                self.waypoint_location.0 + x_distance,
                self.waypoint_location.1 + y_distance,
            );
        }

        fn get_relative_waypoint_position(&self) -> (i32, i32) {
            return (
                self.waypoint_location.0 - self.location.0,
                self.waypoint_location.1 - self.location.1,
            );
        }
    }

    fn parse_actions(input: &mut dyn BufRead) -> Vec<(char, i32)> {
        return input
            .lines()
            .map(|line| line.unwrap())
            .map(|line| parse_action(line))
            .collect();
    }

    fn parse_action(action: String) -> (char, i32) {
        return (
            action[..1].chars().collect::<Vec<char>>()[0],
            action[1..].parse::<i32>().unwrap(),
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::day12;
    use std::{fs::File, io::BufReader};

    mod ship_tests {
        use crate::day12::{Navigator, Ship};

        #[test]
        fn keep_angle_between_0_and_360() {
            let mut ship = Ship::new();
            ship.navigate(vec![('L', 90), ('L', 90)]);
            assert_eq!(ship.get_facing(), 270);
        }
    }

    #[test]
    fn sample_ship() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day12::navigate(&mut f), 25);
    }

    #[test]
    fn day12_ship() {
        let mut f = BufReader::new(File::open("./day12.input").unwrap());
        assert_eq!(day12::navigate(&mut f), 1482);
    }

    #[test]
    fn sample_ship_with_waypoint() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day12::navigate_with_waypoint(&mut f), 286);
    }

    #[test]
    fn day12_ship_with_waypoint() {
        let mut f = BufReader::new(File::open("./day12.input").unwrap());
        assert_eq!(day12::navigate_with_waypoint(&mut f), 48739);
    }
}
