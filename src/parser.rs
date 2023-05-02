use crate::{vm::*, PROGRAM_LENGTH};

pub fn parse_program_string(programstring: &str) -> [Inst; PROGRAM_LENGTH] {
    let mut program: [Inst; PROGRAM_LENGTH] = [Inst::new(InstType::_Push, 0); PROGRAM_LENGTH];
    let mut index = 0;

    for line in programstring.lines() {
        let mut parts = line.trim().split(' ');
        let instype_str = parts.next().unwrap();
        let operand_str = parts.next().unwrap_or("0");

        let instype = match instype_str.to_lowercase().as_str().trim() {
            "push" => InstType::_Push,
            "pop" => InstType::_Pop,
            "plus" => InstType::_Plus,
            "minus" => InstType::_Minus,
            "mul" => InstType::_Mul,
            "div" => InstType::_Div,
            "halt" => InstType::_Halt,
            _ => panic!("Invalid instruction type: {}", instype_str),
        };

        let operand = operand_str.parse().unwrap_or(0);
        program[index] = Inst::new(instype, operand);
        index += 1;
    }

    program
}
