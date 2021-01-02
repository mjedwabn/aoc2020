pub mod day16 {
    use itertools::Itertools;
    use regex::Regex;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::io::BufRead;

    pub fn get_error_rate(input: &mut dyn BufRead) -> i32 {
        let parsed_input = parse_input(input);
        return count_invalid_values(&parsed_input.nearby_tickets, &parsed_input.rules);
    }

    fn count_invalid_values(
        tickets: &Vec<Vec<i32>>,
        rules: &HashMap<String, Vec<(i32, i32)>>,
    ) -> i32 {
        return tickets
            .iter()
            .map(|t| count_invalid_ticket_values(t, rules))
            .sum();
    }

    fn count_invalid_ticket_values(
        ticket: &Vec<i32>,
        rules: &HashMap<String, Vec<(i32, i32)>>,
    ) -> i32 {
        return ticket.iter().filter(|v| is_value_invalid(**v, rules)).sum();
    }

    pub fn get_my_ticket_departures(input: &mut dyn BufRead) -> i64 {
        let parsed = parse_input(input);
        let fields = deduct_fields_order(&parsed.nearby_tickets, &parsed.rules);
        return fields
            .iter()
            .enumerate()
            .filter(|f| f.1.starts_with("departure"))
            .map(|f| parsed.my_ticket[f.0] as i64)
            .fold(1, |a, b| a * b);
    }

    pub fn determine_fields_order(input: &mut dyn BufRead) -> Vec<String> {
        let parsed = parse_input(input);
        return deduct_fields_order(&parsed.nearby_tickets, &parsed.rules);
    }

    fn deduct_fields_order(
        tickets: &Vec<Vec<i32>>,
        rules: &HashMap<String, Vec<(i32, i32)>>,
    ) -> Vec<String> {
        let valid_tickets: Vec<&Vec<i32>> = discard_invalid_tickets(tickets, rules);

        let fields_count = tickets[0].len();
        let mut matching_fields: HashMap<usize, HashSet<&String>> = HashMap::new();
        for i in 0..fields_count {
            let field_values = get_field_values(i, &valid_tickets);
            matching_fields.insert(i, get_matching_fields(&field_values, rules));
        }

        while !all_fields_deducted(&matching_fields) {
            let deducted = get_deducted_fields(&matching_fields);
            for d in deducted {
                remove_field_from_undeducted(d, &mut matching_fields);
            }
        }

        return matching_fields
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
            .flat_map(|f| f.1)
            .map(|f| String::from(f))
            .collect();
    }

    fn all_fields_deducted(matching_fields: &HashMap<usize, HashSet<&String>>) -> bool {
        return !matching_fields.iter().any(|f| f.1.len() > 1);
    }

    fn get_deducted_fields<'a>(
        matching_fields: &HashMap<usize, HashSet<&'a String>>,
    ) -> HashSet<&'a String> {
        return matching_fields
            .iter()
            .filter(|f| f.1.len() == 1)
            .flat_map(|f| f.1)
            .map(|f| *f)
            .collect();
    }

    fn remove_field_from_undeducted(
        field: &String,
        matching_fields: &mut HashMap<usize, HashSet<&String>>,
    ) {
        matching_fields
            .iter_mut()
            .filter(|f| f.1.len() > 1)
            .for_each(|(_, fields)| {
                fields.remove(field);
            });
    }

    fn get_matching_fields<'a>(
        values: &Vec<i32>,
        rules: &'a HashMap<String, Vec<(i32, i32)>>,
    ) -> HashSet<&'a String> {
        return rules
            .iter()
            .filter(|r| values_match_field(values, r.1))
            .map(|r| r.0)
            .collect();
    }

    fn values_match_field(values: &Vec<i32>, field_rules: &Vec<(i32, i32)>) -> bool {
        return values.iter().all(|v| value_match_rules(*v, field_rules));
    }

    fn value_match_rules(value: i32, field_rules: &Vec<(i32, i32)>) -> bool {
        return field_rules.iter().any(|r| value >= r.0 && value <= r.1);
    }

    fn get_field_values(index: usize, tickets: &Vec<&Vec<i32>>) -> Vec<i32> {
        return tickets.iter().map(|t| t[index]).collect();
    }

    fn discard_invalid_tickets<'a>(
        tickets: &'a Vec<Vec<i32>>,
        rules: &HashMap<String, Vec<(i32, i32)>>,
    ) -> Vec<&'a Vec<i32>> {
        return tickets
            .iter()
            .filter(|t| is_ticket_valid(t, rules))
            .collect();
    }

    fn is_ticket_valid(ticket: &Vec<i32>, rules: &HashMap<String, Vec<(i32, i32)>>) -> bool {
        return ticket.iter().all(|v| !is_value_invalid(*v, rules));
    }

    fn is_value_invalid(value: i32, rules: &HashMap<String, Vec<(i32, i32)>>) -> bool {
        let ranges: Vec<&(i32, i32)> = rules.values().flat_map(|v| v).collect();
        return !ranges.iter().any(|r| value >= r.0 && value <= r.1);
    }

    fn parse_input(input: &mut dyn BufRead) -> Input {
        let lines: Vec<String> = input.lines().map(|line| line.unwrap()).collect();
        let segments: Vec<Vec<String>> = lines
            .split(|line| line == "")
            .map(|s| parse_segment(s))
            .collect();

        return Input {
            rules: parse_rules(&segments[0]),
            my_ticket: parse_my_ticket(&segments[1]),
            nearby_tickets: parse_nearby_tickets(&segments[2]),
        };
    }

    fn parse_segment(lines: &[String]) -> Vec<String> {
        return lines.iter().map(|s| String::from(s)).collect();
    }

    fn parse_rules(lines: &Vec<String>) -> HashMap<String, Vec<(i32, i32)>> {
        return lines
            .iter()
            .map(|line| parse_rule(line))
            .collect::<HashMap<_, _>>();
    }

    fn parse_rule(line: &String) -> (String, Vec<(i32, i32)>) {
        let re = Regex::new(
            r"(?P<field>[\w\s]+): (?P<from_1>\d+)-(?P<to_1>\d+) or (?P<from_2>\d+)-(?P<to_2>\d+)",
        )
        .unwrap();

        let caps = re.captures(line).unwrap();
        return (
            String::from(&caps["field"]),
            vec![
                (
                    caps["from_1"].parse::<i32>().unwrap(),
                    caps["to_1"].parse::<i32>().unwrap(),
                ),
                (
                    caps["from_2"].parse::<i32>().unwrap(),
                    caps["to_2"].parse::<i32>().unwrap(),
                ),
            ],
        );
    }

    fn parse_my_ticket(lines: &Vec<String>) -> Vec<i32> {
        let ticket_lines = lines.iter().skip(1).collect::<Vec<&String>>();
        let ticket_line = ticket_lines.first().unwrap();
        return parse_ticket(ticket_line);
    }

    fn parse_nearby_tickets(lines: &Vec<String>) -> Vec<Vec<i32>> {
        let ticket_lines = lines.iter().skip(1).collect::<Vec<&String>>();
        return ticket_lines.iter().map(|line| parse_ticket(line)).collect();
    }

    fn parse_ticket(line: &String) -> Vec<i32> {
        return line.split(",").map(|n| n.parse::<i32>().unwrap()).collect();
    }

    struct Input {
        rules: HashMap<String, Vec<(i32, i32)>>,
        my_ticket: Vec<i32>,
        nearby_tickets: Vec<Vec<i32>>,
    }
}

#[cfg(test)]
mod tests {
    use crate::day16::determine_fields_order;
    use crate::day16::get_error_rate;
    use crate::day16::get_my_ticket_departures;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(get_error_rate(&mut f), 71);
    }

    #[test]
    fn day16_input() {
        let mut f = BufReader::new(File::open("./day16.input").unwrap());
        assert_eq!(get_error_rate(&mut f), 25961);
    }

    #[test]
    fn fields_order() {
        let mut f = BufReader::new(File::open("./part2_sample.input").unwrap());
        assert_eq!(determine_fields_order(&mut f), vec!["row", "class", "seat"]);
    }

    #[test]
    fn day16_my_ticket_fields() {
        let mut f = BufReader::new(File::open("./day16.input").unwrap());
        assert_eq!(get_my_ticket_departures(&mut f), 603409823791);
    }
}
