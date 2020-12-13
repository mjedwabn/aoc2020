pub mod day5 {
    use regex::Regex;
    use std::io::{self, BufRead};
    use std::{fs::File, path::Path};

    pub fn find_highest_seat_id<P>(input_file: P) -> i32
    where
        P: AsRef<Path>,
    {
        let boarding_passes: Vec<String> = read_lines(input_file);
        return boarding_passes
            .iter()
            .map(|p| find_seat_id(p))
            .max()
            .unwrap_or(0);
    }

    fn read_lines<P>(filename: P) -> Vec<String>
    where
        P: AsRef<Path>,
    {
        if let Ok(file) = File::open(filename) {
            return io::BufReader::new(file)
                .lines()
                .map(|v| v.unwrap())
                .collect();
        }
        return Vec::new();
    }

    pub fn find_my_seat_id<P>(input_file: P) -> i32
    where
        P: AsRef<Path>,
    {
        let boarding_passes: Vec<String> = read_lines(input_file);
        let mut seat_ids: Vec<i32> = boarding_passes.iter().map(|p| find_seat_id(p)).collect();

        seat_ids.sort_unstable();

        return seat_ids
            .windows(2)
            .find(|w| w[0] + 2 == w[1])
            .and_then(|w| Some(w[0] + 1))
            .unwrap_or(0);
    }

    pub fn find_seat_id(boarding_pass: &str) -> i32 {
        let location = find_seat(boarding_pass);
        return location.0 * 8 + location.1;
    }

    pub fn find_seat(boarding_pass: &str) -> (i32, i32) {
        let re = Regex::new(r"^([FB]{7})([LR]{3})$").unwrap();
        if let Some(caps) = re.captures(boarding_pass) {
            return (find_row(&caps[1]), find_column(&caps[2]));
        }
        return (0, 0);
    }

    fn find_row(directions: &str) -> i32 {
        let mut range = (0, 127);
        for d in directions.chars() {
            range = bsp(range, d == 'F');
        }

        range.0
    }

    fn find_column(directions: &str) -> i32 {
        let mut range = (0, 7);
        for d in directions.chars() {
            range = bsp(range, d == 'L');
        }

        range.0
    }

    fn bsp(range: (i32, i32), lower: bool) -> (i32, i32) {
        let a = if lower {
            range.0
        } else {
            (range.0 + range.1 + 1) / 2
        };

        let b = if lower {
            (range.0 + range.1 - 1) / 2
        } else {
            range.1
        };

        (a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::day5;

    #[test]
    fn sample_locations() {
        assert_eq!(day5::find_seat("FBFBBFFRLR"), (44, 5));
    }
    #[test]
    fn sample_ids() {
        assert_eq!(day5::find_seat_id("FBFBBFFRLR"), 357);
        assert_eq!(day5::find_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(day5::find_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(day5::find_seat_id("BBFFBBFRLL"), 820);
    }

    #[test]
    fn highest_seat_id() {
        assert_eq!(day5::find_highest_seat_id("./sample.input"), 820);
    }

    #[test]
    fn day5_input() {
        assert_eq!(day5::find_highest_seat_id("./day5.input"), 828);
    }

    #[test]
    fn my_seat() {
        assert_eq!(day5::find_my_seat_id("./day5.input"), 565);
    }
}
