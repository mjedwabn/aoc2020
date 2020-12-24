pub mod day11 {
    use std::io::BufRead;

    pub fn count_occupied_seats(input: &mut dyn BufRead) -> usize {
        let mut wa: WaitingAreaA = make_waiting_area(input);
        wa.model_arriving_people();
        return wa.get_occupied_seats_count();
    }

    pub fn count_first_seen_occupied_seats(input: &mut dyn BufRead) -> usize {
        let mut wa: WaitingAreaB = make_waiting_area_b(input);
        wa.model_arriving_people();
        return wa.get_occupied_seats_count();
    }

    fn parse_layout(input: &mut dyn BufRead) -> Vec<Vec<char>> {
        return input
            .lines()
            .map(|line| line.unwrap())
            .filter(|line| line != "")
            .map(|line| line.chars().collect())
            .collect();
    }

    pub fn make_waiting_area(input: &mut dyn BufRead) -> WaitingAreaA {
        return WaitingAreaA::new(parse_layout(input));
    }

    pub fn make_waiting_area_b(input: &mut dyn BufRead) -> WaitingAreaB {
        return WaitingAreaB::new(parse_layout(input));
    }

    pub struct WaitingAreaA {
        layout: Vec<Vec<char>>,
        x_boundary: i32,
        y_boundary: i32,
    }

    pub struct WaitingAreaB {
        layout: Vec<Vec<char>>,
        x_boundary: i32,
        y_boundary: i32,
    }

    pub trait WaitingAreaAccessors {
        fn get_layout(&self) -> &Vec<Vec<char>>;
        fn set_layout(&mut self, layout: Vec<Vec<char>>);
        fn get_x_boundary(&self) -> i32;
        fn get_y_boundary(&self) -> i32;
    }

    impl WaitingAreaAccessors for WaitingAreaA {
        fn get_layout(&self) -> &Vec<Vec<char>> {
            return &self.layout;
        }
        fn set_layout(&mut self, layout: Vec<Vec<char>>) {
            self.layout = layout;
        }
        fn get_x_boundary(&self) -> i32 {
            return self.x_boundary;
        }
        fn get_y_boundary(&self) -> i32 {
            return self.y_boundary;
        }
    }

    impl WaitingAreaAccessors for WaitingAreaB {
        fn get_layout(&self) -> &Vec<Vec<char>> {
            return &self.layout;
        }
        fn set_layout(&mut self, layout: Vec<Vec<char>>) {
            self.layout = layout;
        }
        fn get_x_boundary(&self) -> i32 {
            return self.x_boundary;
        }
        fn get_y_boundary(&self) -> i32 {
            return self.y_boundary;
        }
    }

    pub trait PubWaitingArea: WaitingAreaAccessors {
        fn get_occupied_seats_count(&self) -> usize {
            return self
                .get_seats()
                .iter()
                .filter(|s| self.is_occupied(&s))
                .count();
        }

        fn model_arriving_people(&mut self) {
            loop {
                let next_layout = self.move_around();
                if next_layout == *self.get_layout() {
                    break;
                } else {
                    self.set_layout(next_layout);
                }
            }
        }

        fn get_seats(&self) -> Vec<(usize, usize)> {
            return self
                .get_layout()
                .iter()
                .enumerate()
                .flat_map(|y| {
                    y.1.iter()
                        .enumerate()
                        .filter(move |x| !self.is_floor(&(x.0, y.0)))
                        .map(move |x| (x.0, y.0))
                })
                .collect();
        }

        fn is_floor(&self, seat: &(usize, usize)) -> bool {
            return self.get_seat_state(seat) == '.';
        }

        fn get_seat_state(&self, seat: &(usize, usize)) -> char {
            return self.get_layout()[seat.1][seat.0];
        }

        fn occupy(&self, seat: &(usize, usize), layout: &mut Vec<Vec<char>>) {
            self.set_seat_state(seat, '#', layout);
        }

        fn empty(&self, seat: &(usize, usize), layout: &mut Vec<Vec<char>>) {
            self.set_seat_state(seat, 'L', layout);
        }

        fn set_seat_state(&self, seat: &(usize, usize), state: char, layout: &mut Vec<Vec<char>>) {
            layout[seat.1][seat.0] = state;
        }

        fn is_occupied(&self, seat: &(usize, usize)) -> bool {
            return self.get_seat_state(seat) == '#';
        }

        fn is_empty(&self, seat: &(usize, usize)) -> bool {
            return self.get_seat_state(seat) == 'L';
        }

        fn get_adjacent_coords(&self, seat: &(usize, usize)) -> Vec<(usize, usize)> {
            let o = (seat.0 as i32, seat.1 as i32);
            let adj = vec![
                (o.0 - 1, o.1 - 1),
                (o.0, o.1 - 1),
                (o.0 + 1, o.1 - 1),
                (o.0 - 1, o.1),
                (o.0 + 1, o.1),
                (o.0 - 1, o.1 + 1),
                (o.0, o.1 + 1),
                (o.0 + 1, o.1 + 1),
            ];
            return adj
                .iter()
                .filter(|c| self.is_in_boundaries(c))
                .map(|c| (c.0 as usize, c.1 as usize))
                .collect();
        }

        fn is_in_boundaries(&self, seat: &(i32, i32)) -> bool {
            return seat.0 >= 0
                && seat.0 < self.get_x_boundary()
                && seat.1 >= 0
                && seat.1 < self.get_y_boundary();
        }

        fn move_around(&self) -> Vec<Vec<char>>;
    }

    impl WaitingAreaA {
        fn new(layout: Vec<Vec<char>>) -> WaitingAreaA {
            WaitingAreaA {
                x_boundary: get_x_boundary(&layout),
                y_boundary: get_y_boundary(&layout),
                layout: layout,
            }
        }

        pub fn occupied_adjacent_seats(&self, seat: &(usize, usize)) -> usize {
            return self
                .get_adjacent_coords(seat)
                .iter()
                .filter(|c| self.is_occupied(*c))
                .count();
        }
    }

    impl PubWaitingArea for WaitingAreaA {
        fn move_around(&self) -> Vec<Vec<char>> {
            let mut next_layout: Vec<Vec<char>> = self.layout.clone();

            for cc in self.get_seats() {
                if self.is_empty(&cc) && self.occupied_adjacent_seats(&cc) == 0 {
                    self.occupy(&cc, &mut next_layout);
                } else if self.is_occupied(&cc) && self.occupied_adjacent_seats(&cc) >= 4 {
                    self.empty(&cc, &mut next_layout);
                }
            }

            return next_layout;
        }
    }

    impl WaitingAreaB {
        fn new(layout: Vec<Vec<char>>) -> WaitingAreaB {
            WaitingAreaB {
                x_boundary: get_x_boundary(&layout),
                y_boundary: get_y_boundary(&layout),
                layout: layout,
            }
        }

        pub fn occupied_first_seen_seats(&self, seat: &(usize, usize)) -> usize {
            let adj = self.get_adjacent_coords(seat);
            let versors: Vec<(i32, i32)> = adj
                .iter()
                .map(|a| (a.0 as i32 - seat.0 as i32, a.1 as i32 - seat.1 as i32))
                .collect();
            return versors
                .iter()
                .map(|v| self.is_first_seen_seat_in_direction_occupied(seat, &v))
                .filter(|&occ| occ)
                .count();
        }

        fn is_first_seen_seat_in_direction_occupied(
            &self,
            seat: &(usize, usize),
            direction: &(i32, i32),
        ) -> bool {
            let mut next_seat = (seat.0 as i32, seat.1 as i32);
            loop {
                next_seat = (next_seat.0 + direction.0, next_seat.1 + direction.1);

                if !self.is_in_boundaries(&next_seat) {
                    return false;
                }

                if self.is_occupied(&(next_seat.0 as usize, next_seat.1 as usize)) {
                    return true;
                } else if self.is_empty(&(next_seat.0 as usize, next_seat.1 as usize)) {
                    return false;
                }
            }
        }
    }

    impl PubWaitingArea for WaitingAreaB {
        fn move_around(&self) -> Vec<Vec<char>> {
            let mut next_layout: Vec<Vec<char>> = self.layout.clone();

            for cc in self.get_seats() {
                if self.is_empty(&cc) && self.occupied_first_seen_seats(&cc) == 0 {
                    self.occupy(&cc, &mut next_layout);
                } else if self.is_occupied(&cc) && self.occupied_first_seen_seats(&cc) >= 5 {
                    self.empty(&cc, &mut next_layout);
                }
            }

            return next_layout;
        }
    }

    fn get_x_boundary(layout: &Vec<Vec<char>>) -> i32 {
        return layout[0].len() as i32;
    }

    fn get_y_boundary(layout: &Vec<Vec<char>>) -> i32 {
        return layout.len() as i32;
    }

    pub trait Debug: WaitingAreaAccessors {
        fn dump_seat_layout(&self) {
            println!("\nlayout:");
            for line in self.get_layout() {
                println!(
                    "{}",
                    line.iter()
                        .map(|&c| c.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                );
            }
        }
    }

    impl Debug for WaitingAreaA {}

    impl Debug for WaitingAreaB {}
}

#[cfg(test)]
mod waiting_area_tests {
    use crate::day11;

    mod occupied_adjacent_seats {
        use super::read_from_string;
        use crate::day11;

        #[test]
        fn empty_area() {
            let wa = day11::make_waiting_area(&mut read_from_string(
                "\n\
                LLL\n\
                LLL\n\
                LLL",
            ));

            assert_eq!(wa.occupied_adjacent_seats(&(1, 1)), 0);
            assert_eq!(wa.occupied_adjacent_seats(&(0, 0)), 0);
            assert_eq!(wa.occupied_adjacent_seats(&(1, 2)), 0);
        }

        #[test]
        fn some_occupied_seats() {
            let wa = day11::make_waiting_area(&mut read_from_string(
                "\n\
                LL#\n\
                L##\n\
                LLL",
            ));

            assert_eq!(wa.occupied_adjacent_seats(&(0, 0)), 1);
            assert_eq!(wa.occupied_adjacent_seats(&(1, 0)), 3);
            assert_eq!(wa.occupied_adjacent_seats(&(2, 2)), 2);
        }

        #[test]
        fn do_not_count_floor() {
            let wa = day11::make_waiting_area(&mut read_from_string(
                "\n\
                ##.\n\
                #L#\n\
                ###",
            ));

            assert_eq!(wa.occupied_adjacent_seats(&(1, 1)), 7);
        }
    }

    mod occupied_first_seen_seats {
        use super::read_from_string;
        use crate::day11;

        #[test]
        fn occupied_in_all_directions() {
            let wa = day11::make_waiting_area_b(&mut read_from_string(
                "\n\
                .......#.\n\
                ...#.....\n\
                .#.......\n\
                .........\n\
                ..#L....#\n\
                ....#....\n\
                .........\n\
                #........\n\
                ...#.....",
            ));

            assert_eq!(wa.occupied_first_seen_seats(&(3, 4)), 8);
        }

        #[test]
        fn cannot_see_through_empty_or_occupied() {
            let wa = day11::make_waiting_area_b(&mut read_from_string(
                "\n\
                .............\n\
                .L.L.#.#.#.#.\n\
                .............",
            ));

            assert_eq!(wa.occupied_first_seen_seats(&(1, 1)), 0);
        }

        #[test]
        fn no_occupied_seat_in_any_direction() {
            let wa = day11::make_waiting_area_b(&mut read_from_string(
                "\n\
                .##.##.\n\
                #.#.#.#\n\
                ##...##\n\
                ...L...\n\
                ##...##\n\
                #.#.#.#\n\
                .##.##.",
            ));

            assert_eq!(wa.occupied_first_seen_seats(&(3, 3)), 0);
        }
        
        #[test]
        fn occy() {
            let wa = day11::make_waiting_area_b(&mut read_from_string(
                "\n\
                #.##.##.##\n\
                #######.##\n\
                #.#.#..#..\n\
                ####.##.##\n\
                #.##.##.##\n\
                #.#####.##\n\
                ..#.#.....\n\
                ##########\n\
                #.######.#\n\
                #.#####.##",
            ));

            assert_eq!(wa.occupied_first_seen_seats(&(9, 1)), 5);
        }
    }

    #[test]
    fn total_occupied_seats() {
        let count = day11::count_occupied_seats(&mut read_from_string(
            "\n\
            LL#\n\
            L##\n\
            .LL",
        ));

        assert_eq!(count, 3);
    }

    fn read_from_string(s: &str) -> &[u8] {
        s.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use crate::day11;
    use std::{fs::File, io::BufReader};

    #[test]
    fn sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day11::count_occupied_seats(&mut f), 37);
    }

    #[test]
    fn day11_input() {
        let mut f = BufReader::new(File::open("./day11.input").unwrap());
        assert_eq!(day11::count_occupied_seats(&mut f), 2494);
    }

    #[test]
    fn sample_input_part2() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day11::count_first_seen_occupied_seats(&mut f), 26);
    }
    
    #[test]
    fn day11_input_part2() {
        let mut f = BufReader::new(File::open("./day11.input").unwrap());
        assert_eq!(day11::count_first_seen_occupied_seats(&mut f), 2306);
    }
}
