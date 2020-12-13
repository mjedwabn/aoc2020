pub mod day6 {
    use std::{collections::HashMap, io::BufRead};

    pub fn identify_questions_total(input: &mut dyn BufRead) -> i32 {
        identify_questions_anyone_answered(input).iter().sum()
    }

    pub fn identify_questions_everyone_answered_total(input: &mut dyn BufRead) -> i32 {
        identify_questions_everyone_answered(input).iter().sum()
    }

    pub fn identify_questions_anyone_answered(input: &mut dyn BufRead) -> Vec<i32> {
        return identify_question(input, identify_questions_which_anyone_answered);
    }

    pub fn identify_questions_everyone_answered(input: &mut dyn BufRead) -> Vec<i32> {
        return identify_question(input, identify_questions_which_everyone_answered);
    }

    pub fn identify_question(input: &mut dyn BufRead, identifier: fn(&Vec<String>) -> i32) -> Vec<i32> {
        let groups = parse_groups(input);

        return groups
            .iter()
            .map(|g| identifier(g))
            .collect();
    }

    fn parse_groups(input: &mut dyn BufRead) -> Vec<Vec<String>> {
        let lines: Vec<String> = input.lines().map(|line| line.unwrap()).collect();
        return lines
            .split(|line| line == "")
            .map(|g| parse_group(g))
            .collect();
    }

    fn parse_group(lines: &[String]) -> Vec<String> {
        return lines.iter().map(|s| String::from(s)).collect();
    }

    fn identify_questions_which_anyone_answered(group: &Vec<String>) -> i32 {
        let mut questions: Vec<char> = group.join("").chars().collect();
        questions.sort();
        questions.dedup();
        return questions.len() as i32;
    }

    fn identify_questions_which_everyone_answered(group: &Vec<String>) -> i32 {
        let group_size = group.len() as i32;
        let stats: HashMap<char, i32> = group.join("").chars().fold(HashMap::new(), |mut hm, c| {
            *hm.entry(c).or_insert(0) += 1;
            hm
        });

        return stats.iter().filter(|q| *q.1 == group_size).count() as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::day6;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn count_single_person_group() {
        assert_eq!(
            day6::identify_questions_anyone_answered(&mut read_from_string("abc")),
            vec![3]
        );
    }

    #[test]
    fn count_multiple_person_group() {
        assert_eq!(
            day6::identify_questions_anyone_answered(&mut read_from_string(
                "abc\n\
                def\n\
                ghi"
            )),
            vec![9]
        );
    }

    #[test]
    fn count_only_unique_questions() {
        assert_eq!(
            day6::identify_questions_anyone_answered(&mut read_from_string(
                "abc\n\
                abd\n\
                ghi"
            )),
            vec![7]
        );
    }

    #[test]
    fn count_many_groups() {
        assert_eq!(
            day6::identify_questions_anyone_answered(&mut read_from_string(
                "abc\n\
                \n\
                a\n\
                b\n\
                c\n\
                \n\
                ab\n\
                ac\n\
                \n\
                a\n\
                a\n\
                a\n\
                a\n\
                \n\
                b"
            )),
            vec![3, 3, 3, 1, 1]
        );
    }

    #[test]
    fn final_count() {
        assert_eq!(
            day6::identify_questions_total(&mut read_from_string(
                "abc\n\
                \n\
                a\n\
                b\n\
                c\n\
                \n\
                ab\n\
                ac\n\
                \n\
                a\n\
                a\n\
                a\n\
                a\n\
                \n\
                b"
            )),
            11
        );
    }

    #[test]
    fn sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day6::identify_questions_total(&mut f), 11);
    }

    #[test]
    fn day6_input() {
        let mut f = BufReader::new(File::open("./day6.input").unwrap());
        assert_eq!(day6::identify_questions_total(&mut f), 6437);
    }

    #[test]
    fn day6_part2_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day6::identify_questions_everyone_answered_total(&mut f), 6);
    }

    #[test]
    fn day6_part2_input() {
        let mut f = BufReader::new(File::open("./day6.input").unwrap());
        assert_eq!(day6::identify_questions_everyone_answered_total(&mut f), 3229);
    }

    fn read_from_string(s: &str) -> &[u8] {
        s.as_bytes()
    }
}
