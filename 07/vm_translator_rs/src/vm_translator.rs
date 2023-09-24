#[derive(Debug, PartialEq)]
pub enum MemorySegment {
    Local,
    Argument,
    This,
    That,
    Constant,
    Static,
    Pointer,
    Temp,
}

mod parser {
    // Takes a VM instruction and parses it into the type of instruction it is
    // as well as its individual components if necessary
    use super::MemorySegment;

    #[derive(Debug, PartialEq)]
    pub enum ParsedVMInstruction {
        Add,
        Sub,
        Neg,
        Eq,
        Gt,
        Lt,
        And,
        Or,
        Not,
        Pop { segment: MemorySegment, idx: String },
        Push { segment: MemorySegment, idx: String },
    }

    pub fn parse_instruction(instruction: &str) -> ParsedVMInstruction {
        let split_instr: Vec<&str> = instruction.split(" ").collect();
        match split_instr[0] {
            "add" => ParsedVMInstruction::Add,
            "sub" => ParsedVMInstruction::Sub,
            "neg" => ParsedVMInstruction::Neg,
            "eq" => ParsedVMInstruction::Eq,
            "gt" => ParsedVMInstruction::Gt,
            "lt" => ParsedVMInstruction::Lt,
            "and" => ParsedVMInstruction::And,
            "or" => ParsedVMInstruction::Or,
            "not" => ParsedVMInstruction::Not,
            "pop" => match split_instr[1] {
                "local" => ParsedVMInstruction::Pop {
                    segment: MemorySegment::Local,
                    idx: String::from(split_instr[2]),
                },
                "argument" => ParsedVMInstruction::Pop {
                    segment: MemorySegment::Argument,
                    idx: String::from(split_instr[2]),
                },
                "this" => ParsedVMInstruction::Pop {
                    segment: MemorySegment::This,
                    idx: String::from(split_instr[2]),
                },
                "that" => ParsedVMInstruction::Pop {
                    segment: MemorySegment::That,
                    idx: String::from(split_instr[2]),
                },
                "constant" => ParsedVMInstruction::Pop {
                    segment: MemorySegment::Constant,
                    idx: String::from(split_instr[2]),
                },
                "static" => ParsedVMInstruction::Pop {
                    segment: MemorySegment::Static,
                    idx: String::from(split_instr[2]),
                },
                "pointer" => ParsedVMInstruction::Pop {
                    segment: MemorySegment::Pointer,
                    idx: String::from(split_instr[2]),
                },
                "temp" => ParsedVMInstruction::Pop {
                    segment: MemorySegment::Temp,
                    idx: String::from(split_instr[2]),
                },
                _ => panic!("Invalid memory segment: {}", split_instr[1]),
            },
            "push" => match split_instr[1] {
                "local" => ParsedVMInstruction::Push {
                    segment: MemorySegment::Local,
                    idx: String::from(split_instr[2]),
                },
                "argument" => ParsedVMInstruction::Push {
                    segment: MemorySegment::Argument,
                    idx: String::from(split_instr[2]),
                },
                "this" => ParsedVMInstruction::Push {
                    segment: MemorySegment::This,
                    idx: String::from(split_instr[2]),
                },
                "that" => ParsedVMInstruction::Push {
                    segment: MemorySegment::That,
                    idx: String::from(split_instr[2]),
                },
                "constant" => ParsedVMInstruction::Push {
                    segment: MemorySegment::Constant,
                    idx: String::from(split_instr[2]),
                },
                "static" => ParsedVMInstruction::Push {
                    segment: MemorySegment::Static,
                    idx: String::from(split_instr[2]),
                },
                "pointer" => ParsedVMInstruction::Push {
                    segment: MemorySegment::Pointer,
                    idx: String::from(split_instr[2]),
                },
                "temp" => ParsedVMInstruction::Push {
                    segment: MemorySegment::Temp,
                    idx: String::from(split_instr[2]),
                },
                _ => panic!("Invalid memory segment: {}", split_instr[1]),
            },
            _ => panic!("Invalid instruction type: {}", split_instr[0]),
        }
    }
}

mod translator {
    // Given a parsed VM instruction, translates the instruction into its
    // valid Hack assembly code
}

#[cfg(test)]
mod tests {
    use super::parser::{parse_instruction, ParsedVMInstruction};
    use super::MemorySegment;

    #[test]
    fn test_parse_valid_instruction() {
        let test_cases = Vec::from([
            (
                "push constant 10",
                ParsedVMInstruction::Push {
                    segment: MemorySegment::Constant,
                    idx: String::from("10"),
                },
            ),
            (
                "pop argument 2",
                ParsedVMInstruction::Pop {
                    segment: MemorySegment::Argument,
                    idx: String::from("2"),
                },
            ),
            ("add", ParsedVMInstruction::Add),
            ("sub", ParsedVMInstruction::Sub),
        ]);

        for test in test_cases {
            let parsed_instruction = parse_instruction(test.0);
            assert_eq!(parsed_instruction, test.1);
        }
    }

    #[test]
    #[should_panic]
    fn test_parse_invalid_instruction() {
        let _parsed_instruction = parse_instruction("gte");
    }

    #[test]
    #[should_panic]
    fn test_parse_invalid_push_instruction() {
        let _parsed_instruction = parse_instruction("push constant");
    }
}
