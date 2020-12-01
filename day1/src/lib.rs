pub mod day1 {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    pub fn fix_report_input<P>(input_file: P) -> i32 
    where P: AsRef<Path>, {
        let input = parse_input(input_file);
        return fix_report(input);
    }

    pub fn fix_report_part2_input<P>(input_file: P) -> i32
    where P: AsRef<Path>, {
        let input = parse_input(input_file);
        return fix_report_part2(input);
    }

    fn parse_input<P>(input_file: P) -> Vec<i32> 
    where P: AsRef<Path>, {
        let mut entries: Vec<i32> = Vec::new();

        if let Ok(lines) = read_lines(input_file) {
            for line in lines {
                if let Ok(entry) = line {
                    let parsed_entry = entry.parse::<i32>().unwrap();
                    entries.push(parsed_entry);
                }
            }
        }

        return entries;
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn fix_report(report: Vec<i32>) -> i32 {    
        for v1 in &report {
            for v2 in &report {
                if v1 + v2 == 2020 {
                    return v1 * v2;
                }
            }
        }

        return 0;
    }

    pub fn fix_report_part2(report: Vec<i32>) -> i32 {    
        for v1 in &report {
            for v2 in &report {
                for v3 in &report {
                    if v1 + v2 + v3 == 2020 {
                        return v1 * v2 * v3;
                    }
                }
            }
        }

        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::day1;

    #[test]
    fn sample_report() {
        assert_eq!(day1::fix_report(vec![1721, 979, 366, 299, 675, 1456]), 514579);
    }

    #[test]
    fn sample_report_part2() {
        assert_eq!(day1::fix_report_part2(vec![1721, 979, 366, 299, 675, 1456]), 241861950);
    }

    #[test]
    fn sample_input() {
        assert_eq!(day1::fix_report_input("./sample.input"), 514579);
    }

    #[test]
    fn day1_input() {
        assert_eq!(day1::fix_report_input("./day1.input"), 1003971);
    }

    #[test]
    fn day1_part2_input() {
        assert_eq!(day1::fix_report_part2_input("./day1.input"), 84035952);
    }
}
