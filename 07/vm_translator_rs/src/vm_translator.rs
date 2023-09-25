use std::fs::{read_to_string, write};

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
    use super::parser::ParsedVMInstruction;
    use super::MemorySegment;

    const ADD: &'static [&str] = &["@SP", "M=M-1", "A=M", "D=M", "A=A-1", "M=M+D"];
    const SUBTRACT: &'static [&str] = &["@SP", "M=M-1", "A=M", "D=M", "A=A-1", "M=M-D"];
    const NEG: &'static [&str] = &["@SP", "A=M-1", "M=-M"];

    fn const_instr_to_vec(const_instr: &'static [&str]) -> Vec<String> {
        const_instr.iter().map(|&s| s.to_string()).collect()
    }

    pub fn translate(instruction: &ParsedVMInstruction) -> Vec<String> {
        match instruction {
            ParsedVMInstruction::Add => const_instr_to_vec(ADD),
            ParsedVMInstruction::Sub => const_instr_to_vec(SUBTRACT),
            ParsedVMInstruction::Neg => const_instr_to_vec(NEG),
            ParsedVMInstruction::Eq => todo!(),
            ParsedVMInstruction::Gt => todo!(),
            ParsedVMInstruction::Lt => todo!(),
            ParsedVMInstruction::And => todo!(),
            ParsedVMInstruction::Or => todo!(),
            ParsedVMInstruction::Not => todo!(),
            ParsedVMInstruction::Pop { segment, idx } => todo!(),
            ParsedVMInstruction::Push { segment, idx } => match segment {
                MemorySegment::Local => todo!(),
                MemorySegment::Argument => todo!(),
                MemorySegment::This => todo!(),
                MemorySegment::That => todo!(),
                MemorySegment::Constant => push_const(idx),
                MemorySegment::Static => todo!(),
                MemorySegment::Pointer => todo!(),
                MemorySegment::Temp => todo!(),
            },
        }
    }

    fn push_const(idx: &String) -> Vec<String> {
        vec![
            format!("@{idx}"),
            String::from("D=A"),
            String::from("@SP"),
            String::from("M=M+1"),
            String::from("A=M-1"),
            String::from("M=D"),
        ]
    }
}

fn read_lines(infile: &str) -> Vec<String> {
    // Reads the lines of the infile, while ignoring comments and whitespace.
    let mut lines = Vec::new();
    for line in read_to_string(infile).unwrap().lines() {
        if let Some(line) = strip_comment_and_whitespace(line) {
            lines.push(line);
        }
    }
    lines
}

fn strip_comment_and_whitespace(line: &str) -> Option<String> {
    let split: Vec<&str> = line.split("//").collect();
    let line = split[0].trim();
    if line.is_empty() {
        return None;
    } else {
        return Some(String::from(line));
    }
}

pub fn write_lines(outfile: &str, asm_output: &Vec<String>) {
    write(outfile, asm_output.join("\n")).expect(&format!(
        "Failed to write hack assembly output to {}",
        outfile
    ));
}

pub fn translate(infile: &str) -> Vec<String> {
    let lines = read_lines(infile);
    let mut asm_output: Vec<String> = Vec::new();
    for line in lines {
        let instruction = parser::parse_instruction(&line);
        let asm = translator::translate(&instruction);
        asm_output.extend(asm);
    }
    asm_output
}

#[cfg(test)]
mod tests {
    use super::parser::{parse_instruction, ParsedVMInstruction};
    use super::MemorySegment;

    #[test]
    fn test_parse_valid_instruction() {
        let test_cases = vec![
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
        ];

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
