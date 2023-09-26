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

impl MemorySegment {
    pub fn seg_ptr(&self) -> &str {
        match *self {
            MemorySegment::Local => "LCL",
            MemorySegment::Argument => "ARG",
            MemorySegment::This => "THIS",
            MemorySegment::That => "THAT",
            _ => panic!("No segment pointer for {:?}", self),
        }
    }
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
        Pop { segment: MemorySegment, idx: u16 },
        Push { segment: MemorySegment, idx: u16 },
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
                    idx: split_instr[2].parse::<u16>().unwrap(),
                },
                "argument" => ParsedVMInstruction::Pop {
                    segment: MemorySegment::Argument,
                    idx: split_instr[2].parse::<u16>().unwrap(),
                },
                "this" => ParsedVMInstruction::Pop {
                    segment: MemorySegment::This,
                    idx: split_instr[2].parse::<u16>().unwrap(),
                },
                "that" => ParsedVMInstruction::Pop {
                    segment: MemorySegment::That,
                    idx: split_instr[2].parse::<u16>().unwrap(),
                },
                "static" => ParsedVMInstruction::Pop {
                    segment: MemorySegment::Static,
                    idx: split_instr[2].parse::<u16>().unwrap(),
                },
                "pointer" => ParsedVMInstruction::Pop {
                    segment: MemorySegment::Pointer,
                    idx: split_instr[2].parse::<u16>().unwrap(),
                },
                "temp" => ParsedVMInstruction::Pop {
                    segment: MemorySegment::Temp,
                    idx: split_instr[2].parse::<u16>().unwrap(),
                },
                _ => panic!("Invalid pop memory segment: {}", split_instr[1]),
            },
            "push" => match split_instr[1] {
                "local" => ParsedVMInstruction::Push {
                    segment: MemorySegment::Local,
                    idx: split_instr[2].parse::<u16>().unwrap(),
                },
                "argument" => ParsedVMInstruction::Push {
                    segment: MemorySegment::Argument,
                    idx: split_instr[2].parse::<u16>().unwrap(),
                },
                "this" => ParsedVMInstruction::Push {
                    segment: MemorySegment::This,
                    idx: split_instr[2].parse::<u16>().unwrap(),
                },
                "that" => ParsedVMInstruction::Push {
                    segment: MemorySegment::That,
                    idx: split_instr[2].parse::<u16>().unwrap(),
                },
                "constant" => ParsedVMInstruction::Push {
                    segment: MemorySegment::Constant,
                    idx: split_instr[2].parse::<u16>().unwrap(),
                },
                "static" => ParsedVMInstruction::Push {
                    segment: MemorySegment::Static,
                    idx: split_instr[2].parse::<u16>().unwrap(),
                },
                "pointer" => ParsedVMInstruction::Push {
                    segment: MemorySegment::Pointer,
                    idx: split_instr[2].parse::<u16>().unwrap(),
                },
                "temp" => ParsedVMInstruction::Push {
                    segment: MemorySegment::Temp,
                    idx: split_instr[2].parse::<u16>().unwrap(),
                },
                _ => panic!("Invalid push memory segment: {}", split_instr[1]),
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

    const ADD: &'static [&str] = &["@SP", "AM=M-1", "D=M", "A=A-1", "M=M+D"];
    const SUBTRACT: &'static [&str] = &["@SP", "AM=M-1", "D=M", "A=A-1", "M=M-D"];
    const NEG: &'static [&str] = &["@SP", "A=M-1", "M=-M"];
    const AND: &'static [&str] = &["@SP", "AM=M-1", "D=M", "A=A-1", "M=D&M"];
    const OR: &'static [&str] = &["@SP", "AM=M-1", "D=M", "A=A-1", "M=D|M"];
    const NOT: &'static [&str] = &["@SP", "A=M-1", "M=!M"];

    const TEMP_OFFSET: u16 = 5;

    fn const_instr_to_vec(const_instr: &'static [&str]) -> Vec<String> {
        const_instr.iter().map(|&s| s.to_string()).collect()
    }

    pub fn translate(
        instruction: ParsedVMInstruction,
        next_instr: usize,
        static_base: &str,
    ) -> Vec<String> {
        match instruction {
            ParsedVMInstruction::Add => const_instr_to_vec(ADD),
            ParsedVMInstruction::Sub => const_instr_to_vec(SUBTRACT),
            ParsedVMInstruction::Neg => const_instr_to_vec(NEG),
            ParsedVMInstruction::Eq => logical_comp(next_instr, "JEQ"),
            ParsedVMInstruction::Gt => logical_comp(next_instr, "JGT"),
            ParsedVMInstruction::Lt => logical_comp(next_instr, "JLT"),
            ParsedVMInstruction::And => const_instr_to_vec(AND),
            ParsedVMInstruction::Or => const_instr_to_vec(OR),
            ParsedVMInstruction::Not => const_instr_to_vec(NOT),
            ParsedVMInstruction::Pop { segment, idx } => match segment {
                MemorySegment::Local => basic_pop(segment, idx),
                MemorySegment::Argument => basic_pop(segment, idx),
                MemorySegment::This => basic_pop(segment, idx),
                MemorySegment::That => basic_pop(segment, idx),
                MemorySegment::Constant => panic!("Invalid instruction: pop constant"),
                MemorySegment::Static => pop_static(idx, static_base),
                MemorySegment::Pointer => pop_ptr(idx),
                MemorySegment::Temp => pop_temp(idx),
            },
            ParsedVMInstruction::Push { segment, idx } => match segment {
                MemorySegment::Local => basic_push(segment, idx),
                MemorySegment::Argument => basic_push(segment, idx),
                MemorySegment::This => basic_push(segment, idx),
                MemorySegment::That => basic_push(segment, idx),
                MemorySegment::Constant => push_const(idx),
                MemorySegment::Static => push_static(idx, static_base),
                MemorySegment::Pointer => push_ptr(idx),
                MemorySegment::Temp => push_temp(idx),
            },
        }
    }

    fn logical_comp(next_instr: usize, jmp_instr: &str) -> Vec<String> {
        vec![
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            String::from("A=A-1"),
            String::from("D=M-D"),
            String::from("M=-1"),
            format!("@{}", next_instr + 11),
            format!("D;{}", jmp_instr),
            String::from("@SP"),
            String::from("A=M-1"),
            String::from("M=0"),
        ]
    }

    fn basic_pop(segment: MemorySegment, idx: u16) -> Vec<String> {
        let seg_ptr = segment.seg_ptr();
        vec![
            format!("@{idx}"),
            String::from("D=A"),
            format!("@{seg_ptr}"),
            String::from("D=D+M"),
            String::from("@13"),
            String::from("M=D"),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            String::from("@13"),
            String::from("A=M"),
            String::from("M=D"),
        ]
    }

    fn pop_temp(idx: u16) -> Vec<String> {
        let mem_addr = TEMP_OFFSET + idx;
        vec![
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            format!("@{mem_addr}"),
            String::from("M=D"),
        ]
    }

    fn pop_ptr(idx: u16) -> Vec<String> {
        let seg_ptr = match idx {
            0 => MemorySegment::This.seg_ptr(),
            1 => MemorySegment::That.seg_ptr(),
            _ => panic!("pop pointer instruction must have index 0 or 1"),
        };
        vec![
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            format!("@{seg_ptr}"),
            String::from("M=D"),
        ]
    }

    fn pop_static(idx: u16, static_base: &str) -> Vec<String> {
        vec![
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            format!("@{static_base}.{idx}"),
            String::from("M=D"),
        ]
    }

    fn push_const(idx: u16) -> Vec<String> {
        vec![
            format!("@{idx}"),
            String::from("D=A"),
            String::from("@SP"),
            String::from("M=M+1"),
            String::from("A=M-1"),
            String::from("M=D"),
        ]
    }

    fn basic_push(segment: MemorySegment, idx: u16) -> Vec<String> {
        let seg_ptr = segment.seg_ptr();
        vec![
            format!("@{idx}"),
            String::from("D=A"),
            format!("@{seg_ptr}"),
            String::from("A=D+M"),
            String::from("D=M"),
            String::from("@SP"),
            String::from("M=M+1"),
            String::from("A=M-1"),
            String::from("M=D"),
        ]
    }

    fn push_temp(idx: u16) -> Vec<String> {
        let mem_addr = TEMP_OFFSET + idx;
        vec![
            format!("@{mem_addr}"),
            String::from("D=M"),
            String::from("@SP"),
            String::from("M=M+1"),
            String::from("A=M-1"),
            String::from("M=D"),
        ]
    }

    fn push_ptr(idx: u16) -> Vec<String> {
        let seg_ptr = match idx {
            0 => MemorySegment::This.seg_ptr(),
            1 => MemorySegment::That.seg_ptr(),
            _ => panic!("push pointer instruction must have index 0 or 1"),
        };
        vec![
            format!("@{seg_ptr}"),
            String::from("D=M"),
            String::from("@SP"),
            String::from("M=M+1"),
            String::from("A=M-1"),
            String::from("M=D"),
        ]
    }

    fn push_static(idx: u16, static_base: &str) -> Vec<String> {
        vec![
            format!("@{static_base}.{idx}"),
            String::from("D=M"),
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
    let static_base: Vec<&str> = infile.split(&['/', '.'][..]).collect();
    let static_base = static_base[static_base.len() - 2]; // Basename of static variables
    let mut asm_output: Vec<String> = Vec::new();
    for line in lines {
        let instruction = parser::parse_instruction(&line);
        let asm = translator::translate(instruction, asm_output.len(), static_base);
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
                    idx: 10,
                },
            ),
            (
                "pop argument 2",
                ParsedVMInstruction::Pop {
                    segment: MemorySegment::Argument,
                    idx: 2,
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
