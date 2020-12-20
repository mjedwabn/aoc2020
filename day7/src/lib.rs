pub mod day7 {
    use regex::Regex;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::io::BufRead;

    pub fn init_airport(input: &mut dyn BufRead) -> Airport {
        let mut airport = Airport::new();
        input
            .lines()
            .map(|rule| parse_rule(&rule.unwrap()))
            .for_each(|r| airport.add_rule(String::from(r.0), r.1));
        return airport;
    }

    fn parse_rule(line: &String) -> (String, HashSet<(String, i32)>) {
        let parts: Vec<&str> = line.split(" bags contain ").collect();
        let color = parts[0];
        if parts[1] == "no other bags." {
            return (String::from(color), HashSet::new());
        } else {
            let content: Vec<&str> = parts[1].split(",").collect();
            let re = Regex::new(r"(\d+) ([\w\s]+) bag?").unwrap();
            let requirements: Vec<(String, i32)> = content
                .iter()
                .map(|c| {
                    let caps = re.captures(c).unwrap();
                    return (String::from(&caps[2]), *&caps[1].parse::<i32>().unwrap());
                })
                .collect();

            return (String::from(color), requirements.iter().cloned().collect());
        }
    }

    pub struct Airport {
        rules: HashMap<String, HashMap<String, i32>>,
    }

    impl Airport {
        pub fn new() -> Airport {
            Airport {
                rules: HashMap::new(),
            }
        }

        fn add_rule(&mut self, color: String, content: HashSet<(String, i32)>) {
            let mut m = HashMap::new();
            content.iter().for_each(|f| {
                m.insert(String::from(&f.0), f.1);
            });
            self.rules.insert(color, m);
        }

        pub fn get_rules(&self) -> Vec<(&str, Vec<(&str, i32)>)> {
            let mut rules: Vec<(&str, Vec<(&str, i32)>)> = self
                .rules
                .iter()
                .map(|r| {
                    return (
                        r.0.as_str(),
                        r.1.iter().map(|c| (c.0.as_str(), *c.1)).collect(),
                    );
                })
                .collect();
            rules.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());
            return rules;
        }

        pub fn how_many_bags_can_contain(&self, bag: &str) -> i32 {
            let mut paths: HashMap<&String, HashSet<&String>> = HashMap::new();
            self.rules.iter().for_each(|r| {
                r.1.iter().for_each(|content| {
                    if !paths.contains_key(content.0) {
                        paths.insert(content.0, HashSet::new());
                    }

                    paths.get_mut(content.0).unwrap().insert(r.0);
                })
            });

            return self.collect_containers(&String::from(bag), &paths).len() as i32;
        }

        fn collect_containers<'a>(
            &self,
            bag: &String,
            paths: &HashMap<&String, HashSet<&'a String>>,
        ) -> HashSet<&'a String> {
            if let Some(out) = paths.get(bag) {
                let outer: HashSet<&String> = out
                    .iter()
                    .map(|o| self.collect_containers(o, paths))
                    .fold(HashSet::new() as HashSet<&String>, |acc, x| {
                        acc.union(&x).map(|e| *e).collect()
                    });
                return out.union(&outer).map(|f| *f).collect();
            } else {
                return HashSet::new();
            }
        }

        pub fn how_many_bags_are_required_inside(&self, bag: &str) -> i32 {
            return self.count_required_bags(&String::from(bag));
        }

        fn count_required_bags(&self, bag: &String) -> i32 {
            if let Some(required) = self.rules.get(bag) {
                return required
                    .iter()
                    .map(|r| self.count_required_bags(r.0) * r.1 + r.1)
                    .sum();
            } else {
                return 0;
            }
        }
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::day7;

    #[test]
    fn bag_containing_single_bag() {
        let airport = day7::init_airport(&mut read_from_string(
            "bright white bags contain 1 shiny gold bag.",
        ));

        assert_eq!(
            airport.get_rules(),
            vec![("bright white", vec![("shiny gold", 1)])]
        );
    }

    #[test]
    fn bag_containing_multiple_bags() {
        let airport = day7::init_airport(&mut read_from_string(
            "bright white bags contain 2 shiny gold bags.",
        ));

        assert_eq!(
            airport.get_rules(),
            vec![("bright white", vec![("shiny gold", 2)])]
        );
    }

    #[test]
    fn bag_containing_multiple_bags_of_many_colors() {
        let airport = day7::init_airport(&mut read_from_string(
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
        ));

        assert_eq!(
            airport.get_rules(),
            vec![("muted yellow", vec![("faded blue", 9), ("shiny gold", 2)])]
        );
    }

    fn read_from_string(s: &str) -> &[u8] {
        s.as_bytes()
    }
}

#[cfg(test)]
mod resolver_tests {
    use crate::day7;
    use std::fs::File;
    use std::io::BufReader;
    #[test]
    fn sample_rules() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        let airport = day7::init_airport(&mut f);

        assert_eq!(airport.how_many_bags_can_contain("shiny gold"), 4);
    }

    #[test]
    fn day7_rules() {
        let mut f = BufReader::new(File::open("./day7.input").unwrap());
        let airport = day7::init_airport(&mut f);

        assert_eq!(airport.how_many_bags_can_contain("shiny gold"), 115);
    }

    #[test]
    fn sample_required_bags() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        let airport = day7::init_airport(&mut f);

        assert_eq!(airport.how_many_bags_are_required_inside("shiny gold"), 32);
    }

    #[test]
    fn sample_required_bags2() {
        let mut f = BufReader::new(File::open("./sample2.input").unwrap());
        let airport = day7::init_airport(&mut f);

        assert_eq!(airport.how_many_bags_are_required_inside("shiny gold"), 126);
    }

    #[test]
    fn day7_required_bags() {
        let mut f = BufReader::new(File::open("./day7.input").unwrap());
        let airport = day7::init_airport(&mut f);

        assert_eq!(
            airport.how_many_bags_are_required_inside("shiny gold"),
            1250
        );
    }
}
