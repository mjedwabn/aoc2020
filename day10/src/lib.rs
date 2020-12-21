pub mod day10 {
    use std::{
        collections::{HashMap, HashSet},
        io::BufRead,
    };

    pub fn jolt_differences(input: &mut dyn BufRead) -> i32 {
        let adapters: Vec<i32> = make_chain(parse_adapters(input));
        let differences: Vec<i32> = get_differences(&adapters);
        let ones = differences.iter().filter(|&&d| d == 1).count();
        let sevens = differences.iter().filter(|&&d| d == 3).count();

        return (ones * sevens) as i32;
    }

    pub fn count_arrangements(input: &mut dyn BufRead) -> i64 {
        let adapters: Vec<i32> = make_chain(parse_adapters(input));
        let graph: HashMap<i32, HashSet<i32>> = make_adapter_graph(&adapters);
        let subchains: Vec<(usize, usize)> = get_relevant_subchains(&adapters);

        let mut paths_count: i64 = 1;
        for chain in subchains {
            paths_count *= count_paths(adapters[chain.0], adapters[chain.1], &graph, 0)
        }

        return paths_count;
    }

    fn parse_adapters(input: &mut dyn BufRead) -> Vec<i32> {
        return input
            .lines()
            .map(|line| line.unwrap())
            .map(|line| line.parse::<i32>().unwrap())
            .collect();
    }

    fn make_chain(mut adapters: Vec<i32>) -> Vec<i32> {
        adapters.append(&mut vec![0, get_buitin_adapter_raiting(&adapters)]);
        adapters.sort();

        return adapters;
    }

    fn get_buitin_adapter_raiting(adapters: &Vec<i32>) -> i32 {
        return adapters.iter().max().unwrap() + 3;
    }

    fn make_adapter_graph(adapters: &Vec<i32>) -> HashMap<i32, HashSet<i32>> {
        let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();

        for a in adapters {
            for c in find_compatible_adapters(*a, &adapters) {
                if !graph.contains_key(a) {
                    graph.insert(*a, HashSet::new());
                }
                graph.get_mut(a).unwrap().insert(*c);
            }
        }

        return graph;
    }

    fn get_relevant_subchains(adapters: &Vec<i32>) -> Vec<(usize, usize)> {
        let mut subchains: Vec<(usize, usize)> = Vec::new();

        let mut start: usize = 0;
        for d in get_differences(&adapters).iter().enumerate() {
            if *d.1 == 3 {
                subchains.push((start, d.0));
                start = d.0 + 1;
            }
        }

        return subchains;
    }

    fn get_differences(adapters: &Vec<i32>) -> Vec<i32> {
        return adapters.windows(2).map(|w| &w[1] - w[0]).collect();
    }

    fn count_paths(s: i32, d: i32, graph: &HashMap<i32, HashSet<i32>>, count: i64) -> i64 {
        let mut path_count = count;
        if s == d {
            path_count += 1;
        } else {
            for n in graph.get(&s).unwrap() {
                path_count = count_paths(*n, d, graph, path_count);
            }
        }
        return path_count;
    }

    fn find_compatible_adapters(outlet: i32, adapters: &Vec<i32>) -> Vec<&i32> {
        return adapters
            .iter()
            .filter(|&&a| a > outlet && a - outlet <= 3)
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use crate::day10;

    #[test]
    fn sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day10::jolt_differences(&mut f), 35);
    }

    #[test]
    fn larger_sample_input() {
        let mut f = BufReader::new(File::open("./larger_sample.input").unwrap());
        assert_eq!(day10::jolt_differences(&mut f), 220);
    }

    #[test]
    fn day10_input() {
        let mut f = BufReader::new(File::open("./day10.input").unwrap());
        assert_eq!(day10::jolt_differences(&mut f), 2100);
    }

    #[test]
    fn sample_arrangements() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day10::count_arrangements(&mut f), 8);
    }

    #[test]
    fn larger_sample_arrangements() {
        let mut f = BufReader::new(File::open("./larger_sample.input").unwrap());
        assert_eq!(day10::count_arrangements(&mut f), 19208);
    }

    #[test]
    fn day10_arrangements() {
        let mut f = BufReader::new(File::open("./day10.input").unwrap());
        assert_eq!(day10::count_arrangements(&mut f), 16198260678656);
    }
}
