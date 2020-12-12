pub mod day4 {
    use regex::Regex;
    use std::io::BufRead;

    const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    pub fn is_passport_valid(passport: &Vec<(String, String)>) -> bool {
        return REQUIRED_FIELDS
            .iter()
            .map(|f| (f, is_field_present(&passport, f)))
            .filter(|f| f.1 == true)
            .count()
            == 7;
    }

    pub fn is_passport_strictly_valid(passport: &Vec<(String, String)>) -> bool {
        return REQUIRED_FIELDS
            .iter()
            .map(|f| get_field(&passport, f))
            .map(|f| f.and_then(|p| Some(is_field_valid(p))).unwrap_or(false))
            .filter(|&f| f == true)
            .count()
            == 7;
    }

    fn get_field<'a>(
        passport: &'a Vec<(String, String)>,
        field: &str,
    ) -> Option<(&'a str, &'a str)> {
        return passport
            .iter()
            .find(|f| f.0 == field)
            .and_then(|f| Some((&f.0[..], &f.1[..])));
    }

    fn is_field_present(passport: &Vec<(String, String)>, field: &str) -> bool {
        return passport.iter().find(|f| f.0 == field) != None;
    }

    pub fn is_field_valid(field: (&str, &str)) -> bool {
        if field.0 == "byr" {
            return is_byr_valid(&field.1);
        } else if field.0 == "iyr" {
            return is_iyr_valid(&field.1);
        } else if field.0 == "eyr" {
            return is_eyr_valid(&field.1);
        } else if field.0 == "hgt" {
            return is_hgt_valid(&field.1);
        } else if field.0 == "hcl" {
            return is_hcl_valid(&field.1);
        } else if field.0 == "ecl" {
            return is_ecl_valid(&field.1);
        } else if field.0 == "pid" {
            return is_pid_valid(&field.1);
        }
        true
    }

    fn is_byr_valid(value: &str) -> bool {
        if let Ok(v) = value.parse::<i32>() {
            return v >= 1920 && v <= 2002;
        }
        false
    }

    fn is_iyr_valid(value: &str) -> bool {
        if let Ok(v) = value.parse::<i32>() {
            return v >= 2010 && v <= 2020;
        }
        false
    }

    fn is_eyr_valid(value: &str) -> bool {
        if let Ok(v) = value.parse::<i32>() {
            return v >= 2020 && v <= 2030;
        }
        false
    }

    fn is_hgt_valid(value: &str) -> bool {
        let re = Regex::new(r"(\d+)(in|cm)").unwrap();
        let optional_caps = re.captures(value);
        if let Some(caps) = optional_caps {
            if &caps[2] == "cm" {
                if let Ok(v) = &caps[1].parse::<i32>() {
                    return *v >= 150 && *v <= 193;
                }
            } else if &caps[2] == "in" {
                if let Ok(v) = &caps[1].parse::<i32>() {
                    return *v >= 59 && *v <= 76;
                }
            }
        }
        false
    }

    fn is_hcl_valid(value: &str) -> bool {
        let re = Regex::new(r"#[a-f0-9]{6}").unwrap();
        return re.is_match(value);
    }

    fn is_ecl_valid(value: &str) -> bool {
        let colours = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        return colours.iter().find(|&&c| c == value) != None;
    }

    fn is_pid_valid(value: &str) -> bool {
        let re = Regex::new(r"\b[0-9]{9}\b").unwrap();
        return re.is_match(value);
    }

    pub fn count_valid_passports(input: &mut dyn BufRead) -> usize {
        return parse_passports(input)
            .iter()
            .map(|p| is_passport_valid(p))
            .filter(|&valid| valid == true)
            .count();
    }

    pub fn count_strictly_valid_passports(input: &mut dyn BufRead) -> usize {
        return parse_passports(input)
            .iter()
            .map(|p| is_passport_strictly_valid(p))
            .filter(|&valid| valid == true)
            .count();
    }

    fn parse_passports(input: &mut dyn BufRead) -> Vec<Vec<(String, String)>> {
        let lines: Vec<String> = input.lines().map(|line| line.unwrap()).collect();
        let passports = lines
            .split(|line| line == "")
            .map(|ll| parse_passport(ll))
            .collect();
        return passports;
    }

    fn parse_passport(lines: &[String]) -> Vec<(String, String)> {
        return lines
            .iter()
            .flat_map(|line| parse_passport_line(line))
            .collect();
    }

    fn parse_passport_line(line: &String) -> Vec<(String, String)> {
        let pairs: Vec<&str> = line.split(' ').collect();
        return pairs.iter().map(|p| parse_pair(p)).collect();
    }

    fn parse_pair(pair: &str) -> (String, String) {
        let parts: Vec<&str> = pair.splitn(2, ":").collect();
        return (String::from(parts[0]), String::from(parts[1]));
    }
}

#[cfg(test)]
mod validator_tests {
    use super::day4;

    #[test]
    fn when_all_fields_are_present_then_passport_is_valid() {
        assert_eq!(
            day4::is_passport_valid(&vec![
                (String::from("ecl"), String::from("gry")),
                (String::from("pid"), String::from("860033327")),
                (String::from("eyr"), String::from("2020")),
                (String::from("hcl"), String::from("#fffffd")),
                (String::from("byr"), String::from("1937")),
                (String::from("iyr"), String::from("2017")),
                (String::from("cid"), String::from("147")),
                (String::from("hgt"), String::from("183cm"))
            ]),
            true
        );
    }

    #[test]
    fn when_not_all_required_fields_are_present_then_passport_is_invalid() {
        assert_eq!(
            day4::is_passport_valid(&vec![
                (String::from("iyr"), String::from("2013")),
                (String::from("ecl"), String::from("amb")),
                (String::from("cid"), String::from("350")),
                (String::from("eyr"), String::from("2023")),
                (String::from("pid"), String::from("028048884")),
                (String::from("hcl"), String::from("cfa07d")),
                (String::from("byr"), String::from("1929"))
            ]),
            false
        );
    }

    #[test]
    fn missing_cid_is_fine() {
        assert_eq!(
            day4::is_passport_valid(&vec![
                (String::from("hcl"), String::from("#ae17e1")),
                (String::from("iyr"), String::from("2013")),
                (String::from("eyr"), String::from("2024")),
                (String::from("ecl"), String::from("brn")),
                (String::from("pid"), String::from("760753108")),
                (String::from("byr"), String::from("1931")),
                (String::from("hgt"), String::from("179cm"))
            ]),
            true
        );
    }

    #[test]
    fn missing_any_other_field_is_not_fine() {
        assert_eq!(
            day4::is_passport_valid(&vec![
                (String::from("hcl"), String::from("#cfa07d")),
                (String::from("eyr"), String::from("2025")),
                (String::from("pid"), String::from("166559648")),
                (String::from("iyr"), String::from("2011")),
                (String::from("ecl"), String::from("brn")),
                (String::from("hgt"), String::from("59in"))
            ]),
            false
        );
    }
}

#[cfg(test)]
mod input_tests {
    use super::day4;
    use std::fs::File;
    use std::io::{BufReader};

    #[test]
    fn fields_separated_by_space() {
        assert_eq!(
            day4::count_valid_passports(&mut read_from_string(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm"
            )),
            1
        );
    }

    #[test]
    fn fields_separated_by_spaces_and_lines() {
        assert_eq!(
            day4::count_valid_passports(&mut read_from_string(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
                byr:1937 iyr:2017 cid:147 hgt:183cm"
            )),
            1
        );
    }

    #[test]
    fn fields_separated_lines() {
        assert_eq!(
            day4::count_valid_passports(&mut read_from_string(
                "ecl:gry\n\
                pid:860033327\n\
                eyr:2020\n\
                hcl:#fffffd\n\
                byr:1937\n\
                iyr:2017\n\
                cid:147\n\
                hgt:183cm"
            )),
            1
        );
    }

    #[test]
    fn multiple_passports() {
        assert_eq!(
            day4::count_valid_passports(&mut read_from_string(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm\n\
                \n\
                ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
                byr:1937 iyr:2017 cid:147 hgt:183cm"
            )),
            2
        );
    }

    #[test]
    fn sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());

        assert_eq!(day4::count_valid_passports(&mut f), 2);
    }

    #[test]
    fn day4_input() {
        let mut f = BufReader::new(File::open("./day4.input").unwrap());

        assert_eq!(day4::count_valid_passports(&mut f), 2);
    }

    fn read_from_string(s: &str) -> &[u8] {
        s.as_bytes()
    }
}

#[cfg(test)]
mod strict_validator_tests {
    use super::day4;
    use std::fs::File;
    use std::io::{BufReader};

    #[test]
    fn birth_year_at_least_1920_and_at_most_2002() {
        assert_eq!(day4::is_field_valid(("byr", "1919")), false);
        assert_eq!(day4::is_field_valid(("byr", "1920")), true);
        assert_eq!(day4::is_field_valid(("byr", "2002")), true);
        assert_eq!(day4::is_field_valid(("byr", "2003")), false);
    }

    #[test]
    fn issue_year_at_least_2010_and_at_most_2020() {
        assert_eq!(day4::is_field_valid(("iyr", "2009")), false);
        assert_eq!(day4::is_field_valid(("iyr", "2010")), true);
        assert_eq!(day4::is_field_valid(("iyr", "2020")), true);
        assert_eq!(day4::is_field_valid(("iyr", "2021")), false);
    }

    #[test]
    fn expiration_year_at_least_2020_and_at_most_2030() {
        assert_eq!(day4::is_field_valid(("eyr", "2019")), false);
        assert_eq!(day4::is_field_valid(("eyr", "2020")), true);
        assert_eq!(day4::is_field_valid(("eyr", "2030")), true);
        assert_eq!(day4::is_field_valid(("eyr", "2031")), false);
    }

    #[test]
    fn height_in_cm_at_least_150_and_at_most_193() {
        assert_eq!(day4::is_field_valid(("hgt", "149cm")), false);
        assert_eq!(day4::is_field_valid(("hgt", "150cm")), true);
        assert_eq!(day4::is_field_valid(("hgt", "193cm")), true);
        assert_eq!(day4::is_field_valid(("hgt", "194cm")), false);
    }

    #[test]
    fn height_in_in_at_least_59_and_at_most_76() {
        assert_eq!(day4::is_field_valid(("hgt", "58in")), false);
        assert_eq!(day4::is_field_valid(("hgt", "59in")), true);
        assert_eq!(day4::is_field_valid(("hgt", "76in")), true);
        assert_eq!(day4::is_field_valid(("hgt", "77in")), false);
    }

    #[test]
    fn height_without_unit_is_invalid() {
        assert_eq!(day4::is_field_valid(("hgt", "60")), false);
    }

    #[test]
    fn hair_color_in_hex() {
        assert_eq!(day4::is_field_valid(("hcl", "#123abc")), true);
        assert_eq!(day4::is_field_valid(("hcl", "#123abz")), false);
        assert_eq!(day4::is_field_valid(("hcl", "123abc")), false);
        assert_eq!(day4::is_field_valid(("hcl", "#abc")), false);
    }

    #[test]
    fn eye_color_matches_selected_colours() {
        assert_eq!(day4::is_field_valid(("ecl", "amb")), true);
        assert_eq!(day4::is_field_valid(("ecl", "blu")), true);
        assert_eq!(day4::is_field_valid(("ecl", "brn")), true);
        assert_eq!(day4::is_field_valid(("ecl", "gry")), true);
        assert_eq!(day4::is_field_valid(("ecl", "grn")), true);
        assert_eq!(day4::is_field_valid(("ecl", "hzl")), true);
        assert_eq!(day4::is_field_valid(("ecl", "oth")), true);

        assert_eq!(day4::is_field_valid(("ecl", "abc")), false);
    }

    #[test]
    fn passport_id_as_nine_digit_number() {
        assert_eq!(day4::is_field_valid(("pid", "000000001")), true);
        assert_eq!(day4::is_field_valid(("pid", "0123456789")), false);
    }

    #[test]
    fn sample_input_part2() {
        let mut f = BufReader::new(File::open("./sample2.input").unwrap());

        assert_eq!(day4::count_strictly_valid_passports(&mut f), 3);
    }

    #[test]
    fn day4_part2_input() {
        let mut f = BufReader::new(File::open("./day4.input").unwrap());

        assert_eq!(day4::count_strictly_valid_passports(&mut f), 198);
    }
}
