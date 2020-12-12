const INPUT: &str = include_str!("../../resources/day08_input.txt");

fn main() {
    let mut computer = Computer::from(INPUT);
    computer.run();

    println!("Day 08 Part 1: {}", computer.accumulator());

    let mut computer = Computer::from(INPUT);
    computer.fix();
    computer.run();
    println!("Day 08 Part 2: {}", computer.accumulator());
}

#[derive(Clone)]
enum Instruction {
    NOP(i32),
    ACC(i32),
    JMP(i32),
}

impl From<&str> for Instruction {
    fn from(item: &str) -> Self {
        let fields: Vec<&str> = item.split(' ').collect();
        let operand = fields[1].parse().expect("Couldn't parse i32");
        match fields[0] {
            "nop" => Instruction::NOP(operand),
            "acc" => Instruction::ACC(operand),
            "jmp" => Instruction::JMP(operand),
            _ => unreachable!("Unknown instruction {}", item),
        }
    }
}

enum RunResult {
    OK,
    LOOP,
}

#[derive(Clone)]
struct Computer {
    accumulator: i32,
    instructions: Vec<Instruction>,
    pointer: i32,
}

impl Computer {
    fn accumulator(&self) -> i32 {
        self.accumulator
    }

    fn run(&mut self) -> RunResult {
        let mut pointer_history = vec![];
        loop {
            if self.pointer as usize >= self.instructions.len() {
                return RunResult::OK;
            }

            if pointer_history.contains(&self.pointer) {
                return RunResult::LOOP;
            } else {
                pointer_history.push(self.pointer)
            }
            let instruction = &self.instructions[self.pointer as usize];
            match instruction {
                Instruction::NOP(_) => self.pointer += 1,
                Instruction::ACC(acc) => {
                    self.accumulator += acc;
                    self.pointer += 1;
                }
                Instruction::JMP(jmp) => self.pointer += jmp,
            }
        }
    }

    fn fix(&mut self) {
        let mut last_pointer_fixed = 0;

        loop {
            let mut computer_clone = self.clone();

            match self.instructions[last_pointer_fixed] {
                Instruction::NOP(op) => {
                    computer_clone.instructions[last_pointer_fixed] = Instruction::JMP(op)
                }
                Instruction::JMP(op) => {
                    computer_clone.instructions[last_pointer_fixed] = Instruction::NOP(op)
                }
                _ => (),
            }

            match computer_clone.run() {
                RunResult::LOOP => last_pointer_fixed += 1,
                RunResult::OK => {
                    self.instructions = computer_clone.instructions;
                    break;
                }
            }
        }
    }
}

impl From<&str> for Computer {
    fn from(item: &str) -> Self {
        Computer {
            accumulator: 0,
            instructions: item.lines().map(|l| Instruction::from(l.trim())).collect(),
            pointer: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6";

        let mut computer = Computer::from(input);
        computer.run();
        assert_eq!(computer.accumulator(), 5);
    }

    #[test]
    fn test2() {
        let input = "nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6";

        let mut computer = Computer::from(input);
        computer.fix();
        computer.run();
        assert_eq!(computer.accumulator(), 8);
    }
}
