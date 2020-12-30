pub mod day13 {
    use std::io::BufRead;

    pub fn find_earliest_bus_score(input: &mut dyn BufRead) -> i32 {
        let (timestamp, buses) = parse_input(input);
        let (bus_id, departure) =
            find_earliest_bus_departure(timestamp, &buses.iter().map(|b| b.1).collect());
        return (departure - timestamp) * bus_id;
    }

    fn find_earliest_bus_departure(timestamp: i32, buses: &Vec<i32>) -> (i32, i32) {
        let mut departures: Vec<(i32, i32)> = buses
            .iter()
            .map(|bus| (*bus, (timestamp / bus) * bus))
            .map(|(bus, div)| (bus, if div == timestamp { div } else { div + bus }))
            .collect();

        departures.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        return departures[0];
    }

    pub fn find_earliest_timestamp_of_subsequent_departures(input: &mut dyn BufRead) -> i64 {
        let (_, buses) = parse_input(input);
        let relations: Vec<Congruence> = buses
            .iter()
            .map(|b| Congruence {
                a: 1,
                b: (b.0 * -1) as i64,
                m: b.1 as i64,
            })
            .collect();

        return solve_congruences(&relations);
    }

    fn solve_congruences(relations: &Vec<Congruence>) -> i64 {
        return solve_congruences_using_chinese_remainder_theorem(&relations);
    }

    fn solve_congruences_using_chinese_remainder_theorem(relations: &Vec<Congruence>) -> i64 {
        let m_mul = relations.iter().map(|c| c.m).fold(1, |a, b| a * b);
        let mi: Vec<i64> = relations.iter().map(|c| m_mul / c.m).collect();
        let mut x: i64 = relations
            .iter()
            .zip(mi.iter())
            .map(|(c, m)| (c, egcd(c.m, *m), m))
            .map(|(c, e, m)| c.b * e.2 * m)
            .sum();
        x = x % m_mul;
        if x < 0 {
            x = x + m_mul;
        }
        return x;
    }

    fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
        if a == 0 {
            return (b, 0, 1);
        }

        let (gcd, x1, y1) = egcd(b % a, a);
        let x = y1 - b / a * x1;
        let y = x1;

        return (gcd, x, y);
    }

    struct Congruence {
        a: i64,
        b: i64,
        m: i64,
    }

    fn parse_input(input: &mut dyn BufRead) -> (i32, Vec<(i32, i32)>) {
        let mut lines = input.lines();
        let timestamp = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
        let buses: Vec<(i32, i32)> = lines
            .next()
            .unwrap()
            .unwrap()
            .split(",")
            .enumerate()
            .filter(|bus| bus.1 != "x")
            .map(|bus| (bus.0 as i32, bus.1.parse::<i32>().unwrap()))
            .collect();
        return (timestamp, buses);
    }
}

#[cfg(test)]
mod tests {
    use crate::day13;
    use std::{fs::File, io::BufReader};

    #[test]
    fn sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day13::find_earliest_bus_score(&mut f), 295);
    }

    #[test]
    fn day13_input() {
        let mut f = BufReader::new(File::open("./day13.input").unwrap());
        assert_eq!(day13::find_earliest_bus_score(&mut f), 3789);
    }

    #[test]
    fn sample_subsequent_buses() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(
            day13::find_earliest_timestamp_of_subsequent_departures(&mut f),
            1068781
        );
    }

    #[test]
    fn other_subsequent_buses() {
        assert_eq!(
            day13::find_earliest_timestamp_of_subsequent_departures(&mut read_from_string(
                "0\n17,x,13,19"
            )),
            3417
        );

        assert_eq!(
            day13::find_earliest_timestamp_of_subsequent_departures(&mut read_from_string(
                "0\n67,7,59,61"
            )),
            754018
        );

        assert_eq!(
            day13::find_earliest_timestamp_of_subsequent_departures(&mut read_from_string(
                "0\n67,x,7,59,61"
            )),
            779210
        );

        assert_eq!(
            day13::find_earliest_timestamp_of_subsequent_departures(&mut read_from_string(
                "0\n67,7,x,59,61"
            )),
            1261476
        );

        assert_eq!(
            day13::find_earliest_timestamp_of_subsequent_departures(&mut read_from_string(
                "0\n1789,37,47,1889"
            )),
            1202161486
        );
    }

    #[test]
    fn day13_subsequent_buses() {
        let mut f = BufReader::new(File::open("./day13.input").unwrap());
        assert_eq!(
            day13::find_earliest_timestamp_of_subsequent_departures(&mut f),
            667437230788118
        );
    }

    fn read_from_string(s: &str) -> &[u8] {
        s.as_bytes()
    }
}
