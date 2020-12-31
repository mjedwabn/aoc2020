pub mod day14 {
    use regex::Regex;
    use std::collections::HashMap;
    use std::io::BufRead;

    pub fn sum_values_in_memory(input: &mut dyn BufRead) -> i64 {
        let mut emulator = EmulatorV1::new();
        let instructions = parse_input(input);
        emulator.interpret_instructions(instructions);
        return emulator.sum_values_in_memory();
    }

    pub fn sum_values_in_memory_v2(input: &mut dyn BufRead) -> i64 {
        let mut emulator = EmulatorV2::new();
        let instructions = parse_input(input);
        emulator.interpret_instructions(instructions);
        return emulator.sum_values_in_memory();
    }

    fn parse_input(input: &mut dyn BufRead) -> Vec<String> {
        return input
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<String>>();
    }

    struct EmulatorV1 {
        mask: Vec<char>,
        memory: HashMap<i64, i64>,
    }

    struct EmulatorV2 {
        mask: Vec<char>,
        memory: HashMap<i64, i64>,
    }

    pub trait DeviceEmulator {
        fn set_bitmask(&mut self, mask: Vec<char>);
        fn set_value(&mut self, addr: i64, value: i64);
        fn get_values(&self) -> Vec<&i64>;
    }

    impl DeviceEmulator for EmulatorV1 {
        fn set_bitmask(&mut self, mask: Vec<char>) {
            self.mask = mask;
        }

        fn set_value(&mut self, addr: i64, value: i64) {
            self.memory.insert(addr, value);
        }

        fn get_values(&self) -> Vec<&i64> {
            return self.memory.values().collect::<Vec<&i64>>();
        }
    }

    impl DeviceEmulator for EmulatorV2 {
        fn set_bitmask(&mut self, mask: Vec<char>) {
            self.mask = mask;
        }

        fn set_value(&mut self, addr: i64, value: i64) {
            self.memory.insert(addr, value);
        }

        fn get_values(&self) -> Vec<&i64> {
            return self.memory.values().collect::<Vec<&i64>>();
        }
    }

    pub trait Emulator: DeviceEmulator {
        fn interpret_instructions(&mut self, instructions: Vec<String>) {
            for i in instructions {
                self.interpret_instruction(i);
            }
        }

        fn interpret_instruction(&mut self, instruction: String) {
            let mask_re = Regex::new(r"mask = ([01X]{36})").unwrap();
            let write_re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

            if let Some(caps) = mask_re.captures(&instruction) {
                self.set_bitmask(caps[1].chars().collect());
            } else if let Some(caps) = write_re.captures(&instruction) {
                let addr: i64 = caps[1].parse::<i64>().unwrap();
                let value: i64 = caps[2].parse::<i64>().unwrap();
                self.write_value(addr, value);
            }
        }

        fn sum_values_in_memory(&self) -> i64 {
            return self.get_values().iter().map(|v| *v).sum();
        }

        fn write_value(&mut self, addr: i64, value: i64);
    }

    impl Emulator for EmulatorV1 {
        fn write_value(&mut self, addr: i64, value: i64) {
            self.set_value(addr, self.apply_mask(value));
        }
    }

    impl EmulatorV1 {
        fn new() -> EmulatorV1 {
            EmulatorV1 {
                mask: Vec::new(),
                memory: HashMap::new(),
            }
        }

        fn apply_mask(&self, value: i64) -> i64 {
            let ones: String = self
                .mask
                .iter()
                .map(|&b| if b == 'X' || b == '0' { '0' } else { '1' })
                .collect();
            let zeros: String = self
                .mask
                .iter()
                .map(|&b| if b == 'X' || b == '1' { '1' } else { '0' })
                .collect();
            let bin_ones = i64::from_str_radix(&ones, 2).unwrap();
            let bin_zeros = i64::from_str_radix(&zeros, 2).unwrap();

            return (value | bin_ones) & bin_zeros;
        }
    }

    impl Emulator for EmulatorV2 {
        fn write_value(&mut self, addr: i64, value: i64) {
            let addresses = self.resolve_addr(addr);
            for a in addresses {
                self.set_value(a, value);
            }
        }
    }

    impl EmulatorV2 {
        fn new() -> EmulatorV2 {
            EmulatorV2 {
                mask: Vec::new(),
                memory: HashMap::new(),
            }
        }

        fn resolve_addr(&self, addr: i64) -> Vec<i64> {
            let base_mask: String = self
                .mask
                .iter()
                .map(|&b| if b == 'X' || b == '1' { '1' } else { '0' })
                .collect();
            let bin_base_mask = i64::from_str_radix(&base_mask, 2).unwrap();
            let x_mask: String = self
                .mask
                .iter()
                .map(|&b| if b == 'X' { '0' } else { '1' })
                .collect();
            let bin_x_mask = i64::from_str_radix(&x_mask, 2).unwrap();
            let base_addr = (addr | bin_base_mask) & bin_x_mask;

            let floating_bits: Vec<i32> = self.get_floating_bits();

            return (0..2i64.pow(floating_bits.len() as u32))
                .map(|n| self.fill_floating_bits(base_addr, n, &floating_bits))
                .collect();
        }

        fn get_floating_bits(&self) -> Vec<i32> {
            let mut floating_bits: Vec<i32> = self
                .mask
                .iter()
                .enumerate()
                .filter(|e| *e.1 == 'X')
                .map(|e| (self.mask.len() - 1 - e.0) as i32)
                .collect();

            floating_bits.sort();

            return floating_bits;
        }

        fn fill_floating_bits(&self, addr: i64, n: i64, floating_bits: &Vec<i32>) -> i64 {
            let mut final_addr = addr;
            for (&floating_pos, n_pos) in floating_bits.iter().zip(0..floating_bits.len() as i32) {
                let bit_pointer_mask = 1 << n_pos;
                let nth_bit = n & bit_pointer_mask;
                let positioned_nth_bit = if floating_pos < n_pos {
                    nth_bit >> n_pos - floating_pos
                } else {
                    nth_bit << floating_pos - n_pos
                };

                final_addr |= positioned_nth_bit;
            }

            return final_addr;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day14::sum_values_in_memory;
    use crate::day14::sum_values_in_memory_v2;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn sample_initialization_program() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(sum_values_in_memory(&mut f), 165);
    }

    #[test]
    fn day14_initialization_program() {
        let mut f = BufReader::new(File::open("./day14.input").unwrap());
        assert_eq!(sum_values_in_memory(&mut f), 10717676595607);
    }

    #[test]
    fn sample_initialization_program_v2() {
        let mut f = BufReader::new(File::open("./floating.input").unwrap());
        assert_eq!(sum_values_in_memory_v2(&mut f), 208);
    }

    #[test]
    fn day14_initialization_program_v2() {
        let mut f = BufReader::new(File::open("./day14.input").unwrap());
        assert_eq!(sum_values_in_memory_v2(&mut f), 3974538275659);
    }
}
