pub mod day15 {
    use std::collections::HashMap;

    pub fn play_2020_game(starting_numbers: Vec<i32>) -> i32 {
        let mut game = MemoryGame::new(starting_numbers, 2020);
        game.play();
        return game.get_last_spoken_number();
    }

    pub fn play_long_game(starting_numbers: Vec<i32>) -> i32 {
        let mut game = MemoryGame::new(starting_numbers, 30000000);
        game.play();
        return game.get_last_spoken_number();
    }

    struct MemoryGame {
        turn: i32,
        number_turns_map: HashMap<i32, Vec<i32>>,
        length: i32,
        last_spoken: i32,
    }

    impl MemoryGame {
        pub fn new(starting_numbers: Vec<i32>, length: i32) -> MemoryGame {
            MemoryGame {
                turn: starting_numbers.len() as i32,
                number_turns_map: starting_numbers
                    .iter()
                    .enumerate()
                    .map(|(i, n)| (*n, vec![(i + 1) as i32]))
                    .collect::<HashMap<_, _>>(),
                length: length,
                last_spoken: starting_numbers[starting_numbers.len() - 1],
            }
        }

        pub fn play(&mut self) {
            while !self.has_finished() {
                self.play_turn();
            }
        }

        fn has_finished(&self) -> bool {
            return self.turn >= self.length;
        }

        fn play_turn(&mut self) {
            let last_spoken_number = self.get_last_spoken_number();
            let age = self.get_number_age(last_spoken_number);
            self.speak(age);
        }

        fn get_number_age(&self, number: i32) -> i32 {
            if let Some(turns) = self.get_previously_spoken_turns(number) {
                if turns.len() < 2 {
                    return 0;
                } else {
                    return turns[turns.len() - 1] - turns[turns.len() - 2];
                }
            } else {
                return 0;
            }
        }

        fn get_previously_spoken_turns(&self, number: i32) -> Option<&Vec<i32>> {
            return self.number_turns_map.get(&number);
        }

        fn speak(&mut self, number: i32) {
            self.turn += 1;
            if self.number_turns_map.contains_key(&number) {
                self.number_turns_map.get_mut(&number).unwrap().push(self.turn);
            } else {
                self.number_turns_map.insert(number, vec![self.turn]);
            }
            self.last_spoken = number;
        }

        pub fn get_last_spoken_number(&self) -> i32 {
            return self.last_spoken;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day15::play_2020_game;
    use crate::day15::play_long_game;

    #[test]
    fn sample_game() {
        assert_eq!(play_2020_game(vec![0, 3, 6]), 436);
    }

    #[test]
    fn other_games() {
        assert_eq!(play_2020_game(vec![1, 3, 2]), 1);
        assert_eq!(play_2020_game(vec![2, 1, 3]), 10);
        assert_eq!(play_2020_game(vec![1, 2, 3]), 27);
        assert_eq!(play_2020_game(vec![2, 3, 1]), 78);
        assert_eq!(play_2020_game(vec![3, 2, 1]), 438);
        assert_eq!(play_2020_game(vec![3, 1, 2]), 1836);
    }

    #[test]
    fn day15_game() {
        assert_eq!(play_2020_game(vec![11, 18, 0, 20, 1, 7, 16]), 639);
    }

    #[test]
    fn long_games() {
        assert_eq!(play_long_game(vec![0, 3, 6]), 175594);
    }

    #[test]
    fn day15_long_game() {
        assert_eq!(play_long_game(vec![11, 18, 0, 20, 1, 7, 16]), 266);
    }
}
