pub mod day2 {
    use std::collections::HashMap;

    pub fn is_password_valid(policy: &PasswordPolicy, password: &String) -> bool {
        let letters: HashMap<char, i32> = password.chars().fold(HashMap::new(), |mut hm, c| {
            *hm.entry(c).or_insert(0) += 1;
            hm
        });
        let occurrences: i32 = *letters.get(&policy.letter).unwrap_or(&0);
        return occurrences >= policy.min && occurrences <= policy.max;
    }

    pub fn is_password_valid2(policy: &PasswordPolicy, password: &String) -> bool {
        let letters: Vec<char> = password.chars().collect();
        let pos1 = policy.min as usize - 1;
        let pos2 = policy.max as usize - 1;

        return xor(
            letters.get(pos1).unwrap() == &policy.letter,
            letters.get(pos2).unwrap() == &policy.letter,
        );
    }

    fn xor(p: bool, q: bool) -> bool {
        return (p || q) && !(p && q);
    }

    pub struct PasswordPolicy {
        pub min: i32,
        pub max: i32,
        pub letter: char,
    }

    pub struct PasswordInput {
        pub policy: PasswordPolicy,
        pub password: String,
    }
}

pub mod day2main {
    use super::day2;
    use regex::Regex;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    pub fn how_many_passwords_are_valid<P>(
        input_file: P,
        validator: fn(&day2::PasswordPolicy, &String) -> bool,
    ) -> usize
    where
        P: AsRef<Path>,
    {
        return parse_input(input_file)
            .iter()
            .map(|input| validator(&input.policy, &input.password))
            .filter(|&valid| valid == true)
            .count();
    }

    fn parse_input<P>(input_file: P) -> Vec<day2::PasswordInput>
    where
        P: AsRef<Path>,
    {
        let mut inputs: Vec<day2::PasswordInput> = Vec::new();
        let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();

        if let Ok(lines) = read_lines(input_file) {
            for line in lines {
                if let Ok(entry) = line {
                    for cap in re.captures_iter(entry.as_str()) {
                        inputs.push(day2::PasswordInput {
                            policy: day2::PasswordPolicy {
                                min: cap[1].parse::<i32>().unwrap(),
                                max: cap[2].parse::<i32>().unwrap(),
                                letter: cap[3].chars().next().unwrap(),
                            },
                            password: String::from(&cap[4]),
                        });
                    }
                }
            }
        }

        return inputs;
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}

#[cfg(test)]
mod tests_part1 {
    use super::day2;
    use super::day2main;

    #[test]
    fn sample_passwords() {
        assert_eq!(
            day2::is_password_valid(
                &day2::PasswordPolicy {
                    min: 1,
                    max: 3,
                    letter: 'a'
                },
                &String::from("abcde")
            ),
            true
        );
        assert_eq!(
            day2::is_password_valid(
                &day2::PasswordPolicy {
                    min: 1,
                    max: 3,
                    letter: 'b'
                },
                &String::from("cdefg")
            ),
            false
        );
        assert_eq!(
            day2::is_password_valid(
                &day2::PasswordPolicy {
                    min: 2,
                    max: 9,
                    letter: 'c'
                },
                &String::from("ccccccccc")
            ),
            true
        );
    }

    #[test]
    fn sample_input() {
        assert_eq!(
            day2main::how_many_passwords_are_valid("./sample.input", day2::is_password_valid),
            2
        );
    }

    #[test]
    fn day2_input() {
        assert_eq!(
            day2main::how_many_passwords_are_valid("./day2.input", day2::is_password_valid),
            564
        );
    }
}

#[cfg(test)]
mod tests_part2 {
    use super::day2;
    use super::day2main;

    #[test]
    fn sample_passwords() {
        assert_eq!(
            day2::is_password_valid2(
                &day2::PasswordPolicy {
                    min: 1,
                    max: 3,
                    letter: 'a'
                },
                &String::from("abcde")
            ),
            true
        );
        assert_eq!(
            day2::is_password_valid2(
                &day2::PasswordPolicy {
                    min: 1,
                    max: 3,
                    letter: 'b'
                },
                &String::from("cdefg")
            ),
            false
        );
        assert_eq!(
            day2::is_password_valid2(
                &day2::PasswordPolicy {
                    min: 2,
                    max: 9,
                    letter: 'c'
                },
                &String::from("ccccccccc")
            ),
            false
        );
    }

    #[test]
    fn sample_input() {
        assert_eq!(
            day2main::how_many_passwords_are_valid("./sample.input", day2::is_password_valid2),
            1
        );
    }

    #[test]
    fn day2_input() {
        assert_eq!(
            day2main::how_many_passwords_are_valid("./day2.input", day2::is_password_valid2),
            325
        );
    }
}
