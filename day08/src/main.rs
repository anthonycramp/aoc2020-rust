const INPUT: &str = include_str!("../../resources/day08_input.txt");

fn main() {
    let mut computer = Computer::from(INPUT);
    computer.run_until_loop();

    println!("Day 08 Part 1: {}", computer.accumulator());
}

enum Instruction {
    NOP(i32),
    ACC(i32),
    JMP(i32),
}

impl From<&str> for Instruction {
    fn from(item: &str) -> Self {
        let fields: Vec<&str> = item.split(" ").collect();
        let operand = fields[1].parse().expect("Couldn't parse i32");
        match fields[0] {
            "nop" => Instruction::NOP(operand),
            "acc" => Instruction::ACC(operand),
            "jmp" => Instruction::JMP(operand),
            _ => unreachable!("Unknown instruction {}", item),
        }
    }
}

struct Computer {
    accumulator: i32,
    instructions: Vec<Instruction>,
    pointer: i32,
}

impl Computer {
    fn accumulator(&self) -> i32 {
        self.accumulator
    }

    fn run_until_loop(&mut self) {
        let mut pointer_history = vec![];
        loop {
            if pointer_history.contains(&self.pointer) {
                break;
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
        computer.run_until_loop();
        assert_eq!(computer.accumulator(), 5);
    }
}
