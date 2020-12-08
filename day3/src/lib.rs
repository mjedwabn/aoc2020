pub mod day3 {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    pub struct Toboggan {
        map: Vec<Vec<char>>,
        current_location: (i32, i32),
        encountered_trees: i64,
    }

    impl Toboggan {
        pub fn new(map: Vec<Vec<char>>) -> Toboggan {
            Toboggan {
                map: map,
                current_location: (0, 0),
                encountered_trees: 0,
            }
        }

        pub fn traverse(&mut self, slope: (i32, i32)) {
            while !self.traversed() {
                self.proceed(slope);
                self.watch();
            }
        }

        fn traversed(&self) -> bool {
            self.current_location.1 as usize >= self.map.len() - 1
        }

        fn proceed(&mut self, slope: (i32, i32)) {
            let map_seed_width = self.map.get(0).unwrap().len() as i32;
            self.current_location.0 = (self.current_location.0 + slope.0) % map_seed_width;
            self.current_location.1 += slope.1;
            // println!("({}, {})", self.current_location.0, self.current_location.1);
        }

        fn watch(&mut self) {
            let row = self.map.get(self.current_location.1 as usize).unwrap();
            let obj = row.get(self.current_location.0 as usize).unwrap();
            // println!("{}", obj);
            if *obj == '#' {
                self.encountered_trees += 1;
            }
        }

        pub fn get_encountered_trees(&self) -> i64 {
            self.encountered_trees
        }
    }

    pub fn traverse_toboggan<P>(input_file: P, slope: (i32, i32)) -> i64
    where
        P: AsRef<Path>,
    {
        let map = parse_input(input_file);
        let mut toboggan = Toboggan::new(map);
        toboggan.traverse(slope);
        return toboggan.get_encountered_trees();
    }

    fn parse_input<P>(input_file: P) -> Vec<Vec<char>>
    where
        P: AsRef<Path>,
    {
        let mut map: Vec<Vec<char>> = Vec::new();

        if let Ok(lines) = read_lines(input_file) {
            for line in lines {
                if let Ok(row) = line {
                    map.push(row.chars().collect());
                }
            }
        }

        return map;
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn pentathlon<P>(input_file: P, slopes: Vec<(i32, i32)>) -> i64
    where
        P: AsRef<Path>,
    {
        return slopes
            .iter()
            .map(|&slope| traverse_toboggan(&input_file, slope))
            .product();
    }
}

#[cfg(test)]
mod tests {
    use super::day3;

    #[test]
    fn sample_input() {
        assert_eq!(day3::traverse_toboggan("./sample.input", (3, 1)), 7);
    }

    #[test]
    fn day3_input() {
        assert_eq!(day3::traverse_toboggan("./day3.input", (3, 1)), 284);
    }

    #[test]
    fn slopes() {
        assert_eq!(day3::traverse_toboggan("./sample.input", (1, 1)), 2);
        assert_eq!(day3::traverse_toboggan("./sample.input", (3, 1)), 7);
        assert_eq!(day3::traverse_toboggan("./sample.input", (5, 1)), 3);
        assert_eq!(day3::traverse_toboggan("./sample.input", (7, 1)), 4);
        assert_eq!(day3::traverse_toboggan("./sample.input", (1, 2)), 2);
    }

    #[test]
    fn sample_final_score() {
        assert_eq!(
            day3::pentathlon(
                "./sample.input",
                vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            ),
            336
        );
    }

    #[test]
    fn day3_final_score() {
        assert_eq!(
            day3::pentathlon(
                "./day3.input",
                vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            ),
            3510149120
        );
    }
}
