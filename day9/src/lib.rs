pub mod day9 {
    use std::io::BufRead;

    pub fn find_weakness(input: &mut dyn BufRead, preamble_size: i32) -> i64 {
        let numbers = parse_input(input);
        let decryptor: XmasDecryptor = XmasDecryptor::new(numbers, preamble_size);
        return decryptor.find_weakness();
    }

    pub fn find_nonmatching_number(input: &mut dyn BufRead, preamble_size: i32) -> i64 {
        let numbers = parse_input(input);
        let decryptor: XmasDecryptor = XmasDecryptor::new(numbers, preamble_size);
        return decryptor.find_first_nonmatching_number();
    }

    fn parse_input(input: &mut dyn BufRead) -> Vec<i64> {
        return input
            .lines()
            .map(|line| line.unwrap())
            .map(|line| line.parse::<i64>().unwrap())
            .collect();
    }

    pub struct XmasDecryptor {
        numbers: Vec<i64>,
        preamble_size: usize,
    }

    impl XmasDecryptor {
        pub fn new(numbers: Vec<i64>, preamble_size: i32) -> XmasDecryptor {
            XmasDecryptor {
                numbers: numbers,
                preamble_size: preamble_size as usize,
            }
        }

        pub fn find_weakness(&self) -> i64 {
            let invalid_number = self.find_first_nonmatching_number();
            let set = self.find_contiguous_set_summing_to(invalid_number);
            let smallest = set.iter().min().unwrap();
            let largest = set.iter().max().unwrap();
            return smallest + largest;
        }

        fn find_contiguous_set_summing_to(&self, sum: i64) -> &[i64] {
            for i in 0..(self.numbers.len() - 1) {
                let mut actual_sum: i64 = self.numbers[i..i+2].iter().sum();

                for j in i+2..(self.numbers.len() - 1) {
                    actual_sum += self.numbers[j];

                    if actual_sum > sum {
                        break;
                    }
                    else if actual_sum == sum {
                        return &self.numbers[i..=j];
                    }
                }
            }

            return &[];
        }

        pub fn find_first_nonmatching_number(&self) -> i64 {
            let a = self.preamble_size;
            let b = self.numbers.len();

            for i in a..b {
                if !self.is_valid_at_index(i) {
                    return self.numbers[i];
                }
            }

            return 0;
        }

        fn is_valid_at_index(&self, index: usize) -> bool {
            let a = index - self.preamble_size;
            let b = index;

            return self.can_be_a_sum(self.numbers[index], (a, b));
        }

        pub fn is_valid_as_next(&self, number: i64) -> bool {
            let a = self.numbers.len() - self.preamble_size;
            let b = self.numbers.len();

            return self.can_be_a_sum(number, (a as usize, b));
        }

        fn can_be_a_sum(&self, number: i64, range: (usize, usize)) -> bool {
            let window = &self.numbers[range.0..range.1];

            for a in window {
                for b in window {
                    if a == b {
                        continue;
                    }

                    if a + b == number {
                        return true;
                    }
                }
            }

            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day9;
    use day9::XmasDecryptor;
    use std::{fs::File, io::BufReader};

    #[test]
    fn sample_checks() {
        let decryptor: XmasDecryptor = XmasDecryptor::new((1..=25).collect(), 25);
        assert_eq!(decryptor.is_valid_as_next(26), true);
        assert_eq!(decryptor.is_valid_as_next(49), true);
        assert_eq!(decryptor.is_valid_as_next(100), false);
        assert_eq!(decryptor.is_valid_as_next(50), false);
    }

    #[test]
    fn window_start() {
        let mut numbers = vec![20];
        numbers.append(&mut (1..20).collect::<Vec<i64>>());
        numbers.append(&mut (21..=25).collect::<Vec<i64>>());
        numbers.append(&mut vec![45]);
        let decryptor: XmasDecryptor = XmasDecryptor::new(numbers, 25);
        assert_eq!(decryptor.is_valid_as_next(26), true);
        assert_eq!(decryptor.is_valid_as_next(65), false);
        assert_eq!(decryptor.is_valid_as_next(64), true);
        assert_eq!(decryptor.is_valid_as_next(66), true);
    }

    #[test]
    fn window_end() {
        let mut numbers = vec![20];
        numbers.append(&mut (1..=26).collect::<Vec<i64>>());
        numbers.append(&mut vec![3000]);
        let decryptor: XmasDecryptor = XmasDecryptor::new(numbers, 25);
        assert_eq!(decryptor.find_first_nonmatching_number(), 3000);
    }

    #[test]
    fn sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day9::find_nonmatching_number(&mut f, 5), 127);
    }

    #[test]
    fn day9_input() {
        let mut f = BufReader::new(File::open("./day9.input").unwrap());
        assert_eq!(day9::find_nonmatching_number(&mut f, 25), 552655238);
    }

    #[test]
    fn sample_weakness() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day9::find_weakness(&mut f, 5), 62);
    }

    #[test]
    fn day9_weakness() {
        let mut f = BufReader::new(File::open("./day9.input").unwrap());
        assert_eq!(day9::find_weakness(&mut f, 25), 70672245);
    }
}
