use std::collections::HashSet;

pub(crate) fn main() {
    use crate::common_io;
    let lines = common_io::read_a_file("day8_instructions.txt").unwrap();
    println!(
        "Infinite loop at {:#?}",
        find_infinite_loop(
            &parse_instructions(lines),
            0,
            &mut Environment { single_global: 0 },
            &mut HashSet::new()
        )
    );
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Nop,
    Acc(i32),
    Jmp(i32),
}

impl Instruction {
    /// Returns 1 in the usual case of moving to the next instruction, otherwise returns the
    /// relative address of the next instruction to run.
    fn eval(&self, environment: &mut Environment) -> i32 {
        match &self {
            Instruction::Nop => 1,
            Instruction::Acc(value) => {
                environment.single_global += value;
                1
            }
            Instruction::Jmp(relative) => *relative,
        }
    }
}

struct Environment {
    single_global: i32,
}

fn find_infinite_loop(
    instructions: &Vec<Instruction>,
    current_instruction_index: usize,
    environment: &mut Environment,
    already_seen: &mut HashSet<usize>,
) -> Option<i32> {
    if already_seen.contains(&current_instruction_index) {
        Some(environment.single_global)
    } else {
        already_seen.insert(current_instruction_index);
        let jump_distance = instructions[current_instruction_index].eval(environment);
        find_infinite_loop(
            instructions,
            (current_instruction_index as i32 + jump_distance) as usize,
            environment,
            already_seen,
        )
    }
}

fn parse_instructions(lines: Vec<String>) -> Vec<Instruction> {
    lines
        .iter()
        .map(|line| {
            let split: Vec<&str> = line.split(" ").collect();
            match split[0] {
                "nop" => Instruction::Nop,
                "acc" => Instruction::Acc(split[1].parse().unwrap()),
                "jmp" => Instruction::Jmp(split[1].parse().unwrap()),
                _ => panic!(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day8::Instruction::Acc;
    use crate::day8::{find_infinite_loop, parse_instructions, Environment};
    use std::collections::HashSet;

    #[test]
    fn can_find_infinite_loop() {
        assert_eq!(
            find_infinite_loop(
                &parse_instructions(vec!(
                    "nop +0".to_string(),
                    "acc +1".to_string(),
                    "jmp +4".to_string(),
                    "acc +3".to_string(),
                    "jmp -3".to_string(),
                    "acc -99".to_string(),
                    "acc +1".to_string(),
                    "jmp -4".to_string(),
                    "acc +6".to_string(),
                )),
                0,
                &mut Environment { single_global: 0 },
                &mut HashSet::new()
            ),
            Some(&Acc(1))
        );
    }
}
