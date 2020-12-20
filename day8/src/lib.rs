pub mod day8 {
    use std::collections::HashMap;
    use std::io::BufRead;

    pub fn run_boot_code(input: &mut dyn BufRead) -> i32 {
        let mut console = GameConsole::new(parse_instructions(input));
        console.run_boot_code();
        return console.get_accumulator();
    }

    pub fn fix_boot_code(input: &mut dyn BufRead) -> i32 {
        let instructions = parse_instructions(input);
        let suspected_instructions: Vec<(i32, String)> = instructions
            .iter()
            .enumerate()
            .filter(|i| i.1.0 == "jmp" || i.1.0 == "nop")
            .map(|i| (i.0 as i32, String::from(&i.1 .0)))
            .collect();

        let results: Vec<(i32, bool)> = suspected_instructions
            .iter()
            .map(|i| apply_fix(&instructions,(i.0, String::from(&i.1))))
            .map(|patched_instructions| {
                let mut console = GameConsole::new(patched_instructions);
                console.run_boot_code();
                return (console.get_accumulator(), console.program_terminated());
            })
            .filter(|result| result.1 == true)
            .collect();

        results[0].0
    }

    fn apply_fix(
        instructions: &Vec<(String, i32)>,
        suspected_instruction: (i32, String),
    ) -> Vec<(String, i32)> {
        let mut copy: Vec<(String, i32)> = instructions.iter().cloned().collect();
        let replacement: String = if suspected_instruction.1 == "jmp" {String::from("nop")} else {String::from("jmp")};
        copy[suspected_instruction.0 as usize] = (replacement, copy[suspected_instruction.0 as usize].1);
        return copy;
    }

    fn parse_instructions(input: &mut dyn BufRead) -> Vec<(String, i32)> {
        return input
            .lines()
            .map(|line| parse_instruction(&line.unwrap()))
            .collect();
    }

    fn parse_instruction(instruction: &String) -> (String, i32) {
        let parts: Vec<&str> = instruction.split(" ").collect();
        return (String::from(parts[0]), parts[1].parse::<i32>().unwrap());
    }

    struct GameConsole {
        instructions: Vec<(String, i32)>,
        accumulator: i32,
        debugger: HashMap<i32, i32>,
        ptr: i32,
    }

    impl GameConsole {
        fn new(instructions: Vec<(String, i32)>) -> GameConsole {
            GameConsole {
                instructions: instructions,
                accumulator: 0,
                debugger: HashMap::new(),
                ptr: 0,
            }
        }

        fn run_boot_code(&mut self) {
            self.move_ptr(0);

            while !self.program_terminated() {
                if !self.before_second_iteration_loop() {
                    break;
                }

                self.execute_instruction();
            }
        }

        fn before_second_iteration_loop(&self) -> bool {
            return *self.debugger.get(&self.ptr).unwrap_or(&0) < 2;
        }

        fn move_ptr(&mut self, offset: i32) {
            self.ptr += offset;
            self.debugger
                .insert(self.ptr, self.debugger.get(&self.ptr).unwrap_or(&0) + 1);
        }

        fn execute_instruction(&mut self) {
            if let Some(instruction) = self.instructions.get(self.ptr as usize) {
                if instruction.0 == "nop" {
                    self.move_ptr(1);
                } else if instruction.0 == "acc" {
                    self.accumulator += instruction.1;
                    self.move_ptr(1);
                } else if instruction.0 == "jmp" {
                    let offset = instruction.1;
                    self.move_ptr(offset);
                }
            }
        }

        pub fn get_accumulator(&self) -> i32 {
            return self.accumulator;
        }

        pub fn program_terminated(&self) -> bool {
            return self.ptr >= self.instructions.len() as i32;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day8;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day8::run_boot_code(&mut f), 5);
    }

    #[test]
    fn day8_input() {
        let mut f = BufReader::new(File::open("./day8.input").unwrap());
        assert_eq!(day8::run_boot_code(&mut f), 1563);
    }

    #[test]
    fn fix_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day8::fix_boot_code(&mut f), 8);
    }

    #[test]
    fn fix_day8_input() {
        let mut f = BufReader::new(File::open("./day8.input").unwrap());
        assert_eq!(day8::fix_boot_code(&mut f), 767);
    }
}
