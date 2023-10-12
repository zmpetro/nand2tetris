use std::fs::read_to_string;
use std::path::Path;

use parser::ParsedVMInstruction;
use translator::{add_instr, call};

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
        Label { label: String },
        Goto { label: String },
        IfGoto { label: String },
        Function { name: String, num_local_vars: u16 },
        Call { name: String, num_args: u16 },
        Return,
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
            "label" => ParsedVMInstruction::Label {
                label: split_instr[1].to_owned(),
            },
            "goto" => ParsedVMInstruction::Goto {
                label: split_instr[1].to_owned(),
            },
            "if-goto" => ParsedVMInstruction::IfGoto {
                label: split_instr[1].to_owned(),
            },
            "function" => ParsedVMInstruction::Function {
                name: split_instr[1].to_owned(),
                num_local_vars: split_instr[2].parse::<u16>().unwrap(),
            },
            "call" => ParsedVMInstruction::Call {
                name: split_instr[1].to_owned(),
                num_args: split_instr[2].parse::<u16>().unwrap(),
            },
            "return" => ParsedVMInstruction::Return,
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
    const RETURN: &'static [&str] = &[
        "@LCL", "D=M", "@5", "M=D", "@5", "D=A", "@5", "A=M-D", "D=M", "@6", "M=D", "@SP", "A=M-1",
        "D=M", "@ARG", "A=M", "M=D", "@ARG", "D=M+1", "@SP", "M=D", "@5", "AM=M-1", "D=M", "@THAT",
        "M=D", "@5", "AM=M-1", "D=M", "@THIS", "M=D", "@5", "AM=M-1", "D=M", "@ARG", "M=D", "@5",
        "AM=M-1", "D=M", "@LCL", "M=D", "@6", "A=M", "0;JMP",
    ];

    const TEMP_OFFSET: u16 = 5;

    fn const_instr_to_vec(next_instr: &mut u16, const_instr: &'static [&str]) -> Vec<String> {
        let mut asm: Vec<String> = vec![];
        for &instr in const_instr {
            add_instr(instr, &mut asm, next_instr)
        }
        asm
    }

    pub fn translate(
        instruction: &ParsedVMInstruction,
        next_instr: &mut u16,
        static_base: &str,
        call_counter: u16,
    ) -> Vec<String> {
        match instruction {
            ParsedVMInstruction::Add => const_instr_to_vec(next_instr, ADD),
            ParsedVMInstruction::Sub => const_instr_to_vec(next_instr, SUBTRACT),
            ParsedVMInstruction::Neg => const_instr_to_vec(next_instr, NEG),
            ParsedVMInstruction::Eq => logical_comp(next_instr, "JEQ"),
            ParsedVMInstruction::Gt => logical_comp(next_instr, "JGT"),
            ParsedVMInstruction::Lt => logical_comp(next_instr, "JLT"),
            ParsedVMInstruction::And => const_instr_to_vec(next_instr, AND),
            ParsedVMInstruction::Or => const_instr_to_vec(next_instr, OR),
            ParsedVMInstruction::Not => const_instr_to_vec(next_instr, NOT),
            ParsedVMInstruction::Pop { segment, idx } => match segment {
                MemorySegment::Local => basic_pop(next_instr, segment, idx),
                MemorySegment::Argument => basic_pop(next_instr, segment, idx),
                MemorySegment::This => basic_pop(next_instr, segment, idx),
                MemorySegment::That => basic_pop(next_instr, segment, idx),
                MemorySegment::Constant => panic!("Invalid instruction: pop constant"),
                MemorySegment::Static => pop_static(next_instr, idx, static_base),
                MemorySegment::Pointer => pop_ptr(next_instr, idx),
                MemorySegment::Temp => pop_temp(next_instr, idx),
            },
            ParsedVMInstruction::Push { segment, idx } => match segment {
                MemorySegment::Local => basic_push(next_instr, segment, idx),
                MemorySegment::Argument => basic_push(next_instr, segment, idx),
                MemorySegment::This => basic_push(next_instr, segment, idx),
                MemorySegment::That => basic_push(next_instr, segment, idx),
                MemorySegment::Constant => push_const(next_instr, idx),
                MemorySegment::Static => push_static(next_instr, idx, static_base),
                MemorySegment::Pointer => push_ptr(next_instr, idx),
                MemorySegment::Temp => push_temp(next_instr, idx),
            },
            ParsedVMInstruction::Label { label } => label_fn(next_instr, &label),
            ParsedVMInstruction::Goto { label } => goto(next_instr, &label),
            ParsedVMInstruction::IfGoto { label } => if_goto(next_instr, &label),
            ParsedVMInstruction::Function {
                name,
                num_local_vars,
            } => function(next_instr, &name, *num_local_vars),
            ParsedVMInstruction::Call { name, num_args } => {
                call(next_instr, &name, *num_args, call_counter)
            }
            ParsedVMInstruction::Return => const_instr_to_vec(next_instr, RETURN),
        }
    }

    pub fn add_instr(instr: &str, asm: &mut Vec<String>, next_instr: &mut u16) {
        // Adds an instruction to a vector of instructions and increments the
        // next_instr counter based on whether it's a label or not
        asm.push(String::from(instr));
        if instr.chars().next().unwrap() != '(' {
            *next_instr += 1;
        }
    }

    fn logical_comp(next_instr: &mut u16, jmp_instr: &str) -> Vec<String> {
        let mut asm: Vec<String> = vec![];
        add_instr("@SP", &mut asm, next_instr);
        add_instr("AM=M-1", &mut asm, next_instr);
        add_instr("D=M", &mut asm, next_instr);
        add_instr("A=A-1", &mut asm, next_instr);
        add_instr("D=M-D", &mut asm, next_instr);
        add_instr("M=-1", &mut asm, next_instr);
        // next_instr + 5 is how many instructions until the end of the current asm block
        add_instr(&format!("@{}", *next_instr + 5), &mut asm, next_instr);
        add_instr(&format!("D;{}", jmp_instr), &mut asm, next_instr);
        add_instr("@SP", &mut asm, next_instr);
        add_instr("A=M-1", &mut asm, next_instr);
        add_instr("M=0", &mut asm, next_instr);
        asm
    }

    fn basic_pop(next_instr: &mut u16, segment: &MemorySegment, idx: &u16) -> Vec<String> {
        let seg_ptr = segment.seg_ptr();
        let mut asm: Vec<String> = vec![];
        add_instr(&format!("@{idx}"), &mut asm, next_instr);
        add_instr("D=A", &mut asm, next_instr);
        add_instr(&format!("@{seg_ptr}"), &mut asm, next_instr);
        add_instr("D=D+M", &mut asm, next_instr);
        add_instr("@SP", &mut asm, next_instr);
        add_instr("AM=M-1", &mut asm, next_instr);
        add_instr("D=D+M", &mut asm, next_instr);
        add_instr("A=D-M", &mut asm, next_instr);
        add_instr("M=D-A", &mut asm, next_instr);
        asm
    }

    fn pop_temp(next_instr: &mut u16, idx: &u16) -> Vec<String> {
        let mem_addr = TEMP_OFFSET + idx;
        let mut asm: Vec<String> = vec![];
        add_instr("@SP", &mut asm, next_instr);
        add_instr("AM=M-1", &mut asm, next_instr);
        add_instr("D=M", &mut asm, next_instr);
        add_instr(&format!("@{mem_addr}"), &mut asm, next_instr);
        add_instr("M=D", &mut asm, next_instr);
        asm
    }

    fn pop_ptr(next_instr: &mut u16, idx: &u16) -> Vec<String> {
        let seg_ptr = match idx {
            0 => MemorySegment::This.seg_ptr(),
            1 => MemorySegment::That.seg_ptr(),
            _ => panic!("pop pointer instruction must have index 0 or 1"),
        };
        let mut asm: Vec<String> = vec![];
        add_instr("@SP", &mut asm, next_instr);
        add_instr("AM=M-1", &mut asm, next_instr);
        add_instr("D=M", &mut asm, next_instr);
        add_instr(&format!("@{seg_ptr}"), &mut asm, next_instr);
        add_instr("M=D", &mut asm, next_instr);
        asm
    }

    fn pop_static(next_instr: &mut u16, idx: &u16, static_base: &str) -> Vec<String> {
        let mut asm: Vec<String> = vec![];
        add_instr("@SP", &mut asm, next_instr);
        add_instr("AM=M-1", &mut asm, next_instr);
        add_instr("D=M", &mut asm, next_instr);
        add_instr(&format!("@{static_base}.{idx}"), &mut asm, next_instr);
        add_instr("M=D", &mut asm, next_instr);
        asm
    }

    fn push_const(next_instr: &mut u16, idx: &u16) -> Vec<String> {
        let mut asm: Vec<String> = vec![];
        add_instr(&format!("@{idx}"), &mut asm, next_instr);
        add_instr("D=A", &mut asm, next_instr);
        add_instr("@SP", &mut asm, next_instr);
        add_instr("M=M+1", &mut asm, next_instr);
        add_instr("A=M-1", &mut asm, next_instr);
        add_instr("M=D", &mut asm, next_instr);
        asm
    }

    fn basic_push(next_instr: &mut u16, segment: &MemorySegment, idx: &u16) -> Vec<String> {
        let seg_ptr = segment.seg_ptr();
        let mut asm: Vec<String> = vec![];
        add_instr(&format!("@{idx}"), &mut asm, next_instr);
        add_instr("D=A", &mut asm, next_instr);
        add_instr(&format!("@{seg_ptr}"), &mut asm, next_instr);
        add_instr("A=D+M", &mut asm, next_instr);
        add_instr("D=M", &mut asm, next_instr);
        add_instr("@SP", &mut asm, next_instr);
        add_instr("M=M+1", &mut asm, next_instr);
        add_instr("A=M-1", &mut asm, next_instr);
        add_instr("M=D", &mut asm, next_instr);
        asm
    }

    fn push_temp(next_instr: &mut u16, idx: &u16) -> Vec<String> {
        let mem_addr = TEMP_OFFSET + idx;
        let mut asm: Vec<String> = vec![];
        add_instr(&format!("@{mem_addr}"), &mut asm, next_instr);
        add_instr("D=M", &mut asm, next_instr);
        add_instr("@SP", &mut asm, next_instr);
        add_instr("M=M+1", &mut asm, next_instr);
        add_instr("A=M-1", &mut asm, next_instr);
        add_instr("M=D", &mut asm, next_instr);
        asm
    }

    fn push_ptr(next_instr: &mut u16, idx: &u16) -> Vec<String> {
        let seg_ptr = match idx {
            0 => MemorySegment::This.seg_ptr(),
            1 => MemorySegment::That.seg_ptr(),
            _ => panic!("push pointer instruction must have index 0 or 1"),
        };
        let mut asm: Vec<String> = vec![];
        add_instr(&format!("@{seg_ptr}"), &mut asm, next_instr);
        add_instr("D=M", &mut asm, next_instr);
        add_instr("@SP", &mut asm, next_instr);
        add_instr("M=M+1", &mut asm, next_instr);
        add_instr("A=M-1", &mut asm, next_instr);
        add_instr("M=D", &mut asm, next_instr);
        asm
    }

    fn push_static(next_instr: &mut u16, idx: &u16, static_base: &str) -> Vec<String> {
        let mut asm: Vec<String> = vec![];
        add_instr(&format!("@{static_base}.{idx}"), &mut asm, next_instr);
        add_instr("D=M", &mut asm, next_instr);
        add_instr("@SP", &mut asm, next_instr);
        add_instr("M=M+1", &mut asm, next_instr);
        add_instr("A=M-1", &mut asm, next_instr);
        add_instr("M=D", &mut asm, next_instr);
        asm
    }

    fn label_fn(next_instr: &mut u16, label: &str) -> Vec<String> {
        let mut asm: Vec<String> = vec![];
        add_instr(&format!("({label})"), &mut asm, next_instr);
        asm
    }

    fn goto(next_instr: &mut u16, label: &str) -> Vec<String> {
        let mut asm: Vec<String> = vec![];
        add_instr(&format!("@{label}"), &mut asm, next_instr);
        add_instr("0;JMP", &mut asm, next_instr);
        asm
    }

    fn if_goto(next_instr: &mut u16, label: &str) -> Vec<String> {
        let mut asm: Vec<String> = vec![];
        add_instr("@SP", &mut asm, next_instr);
        add_instr("AM=M-1", &mut asm, next_instr);
        add_instr("D=M", &mut asm, next_instr);
        add_instr(&format!("@{label}"), &mut asm, next_instr);
        add_instr("D;JNE", &mut asm, next_instr);
        asm
    }

    fn function(next_instr: &mut u16, name: &str, num_local_vars: u16) -> Vec<String> {
        let mut asm: Vec<String> = vec![];
        add_instr(&format!("({name})"), &mut asm, next_instr);
        for _ in 0..num_local_vars {
            add_instr("@SP", &mut asm, next_instr);
            add_instr("M=M+1", &mut asm, next_instr);
            add_instr("A=M-1", &mut asm, next_instr);
            add_instr("M=0", &mut asm, next_instr);
        }
        asm
    }

    pub fn call(next_instr: &mut u16, name: &str, num_args: u16, call_counter: u16) -> Vec<String> {
        let return_addr_label = format!("{name}$ret.{call_counter}");
        let arg_offset = 5 + num_args;
        let mut asm: Vec<String> = vec![];
        add_instr(&format!("@{return_addr_label}"), &mut asm, next_instr);
        add_instr("D=A", &mut asm, next_instr);
        add_instr("@SP", &mut asm, next_instr);
        add_instr("M=M+1", &mut asm, next_instr);
        add_instr("A=M-1", &mut asm, next_instr);
        add_instr("M=D", &mut asm, next_instr);
        add_instr("@LCL", &mut asm, next_instr);
        add_instr("D=M", &mut asm, next_instr);
        add_instr("@SP", &mut asm, next_instr);
        add_instr("M=M+1", &mut asm, next_instr);
        add_instr("A=M-1", &mut asm, next_instr);
        add_instr("M=D", &mut asm, next_instr);
        add_instr("@ARG", &mut asm, next_instr);
        add_instr("D=M", &mut asm, next_instr);
        add_instr("@SP", &mut asm, next_instr);
        add_instr("M=M+1", &mut asm, next_instr);
        add_instr("A=M-1", &mut asm, next_instr);
        add_instr("M=D", &mut asm, next_instr);
        add_instr("@THIS", &mut asm, next_instr);
        add_instr("D=M", &mut asm, next_instr);
        add_instr("@SP", &mut asm, next_instr);
        add_instr("M=M+1", &mut asm, next_instr);
        add_instr("A=M-1", &mut asm, next_instr);
        add_instr("M=D", &mut asm, next_instr);
        add_instr("@THAT", &mut asm, next_instr);
        add_instr("D=M", &mut asm, next_instr);
        add_instr("@SP", &mut asm, next_instr);
        add_instr("M=M+1", &mut asm, next_instr);
        add_instr("A=M-1", &mut asm, next_instr);
        add_instr("M=D", &mut asm, next_instr);
        add_instr(&format!("@{arg_offset}"), &mut asm, next_instr);
        add_instr("D=A", &mut asm, next_instr);
        add_instr("@SP", &mut asm, next_instr);
        add_instr("D=M-D", &mut asm, next_instr);
        add_instr("@ARG", &mut asm, next_instr);
        add_instr("M=D", &mut asm, next_instr);
        add_instr("@SP", &mut asm, next_instr);
        add_instr("D=M", &mut asm, next_instr);
        add_instr("@LCL", &mut asm, next_instr);
        add_instr("M=D", &mut asm, next_instr);
        add_instr(&format!("@{name}"), &mut asm, next_instr);
        add_instr("0;JMP", &mut asm, next_instr);
        add_instr(&format!("({return_addr_label})"), &mut asm, next_instr);
        asm
    }
}

fn read_lines(infile: &Path) -> Vec<String> {
    // Reads the lines of the infile, while ignoring comments and whitespace.
    read_to_string(infile)
        .unwrap()
        .lines()
        .filter_map(|line| strip_comment_and_whitespace(line))
        .collect()
}

fn strip_comment_and_whitespace(line: &str) -> Option<String> {
    let line = line.split("//").next().unwrap().trim();
    if line.is_empty() {
        return None;
    } else {
        return Some(line.to_owned());
    }
}

pub fn translate(infile: &Path, next_instr: &mut u16, call_counter: &mut u16) -> Vec<String> {
    let lines = read_lines(infile);
    let static_base = infile.file_stem().unwrap().to_str().unwrap();
    let mut asm_output: Vec<String> = Vec::new();
    for line in lines {
        let instruction = parser::parse_instruction(&line);
        let asm = translator::translate(&instruction, next_instr, static_base, *call_counter);
        asm_output.extend(asm);
        if let ParsedVMInstruction::Call { .. } = instruction {
            *call_counter += 1;
        }
    }
    asm_output
}

fn get_bootstrap(next_instr: &mut u16) -> Vec<String> {
    let mut bootstrap: Vec<String> = vec![];
    add_instr("@256", &mut bootstrap, next_instr);
    add_instr("D=A", &mut bootstrap, next_instr);
    add_instr("@SP", &mut bootstrap, next_instr);
    add_instr("M=D", &mut bootstrap, next_instr);
    bootstrap.extend(call(next_instr, "Sys.init", 0, 0));
    bootstrap
}

pub fn translate_directory(directory: &Path) -> Vec<String> {
    let mut next_instr: u16 = 0;
    let mut call_counter: u16 = 0;
    let mut asm_output = get_bootstrap(&mut next_instr);
    for entry in directory.read_dir().unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().unwrap() == "vm" {
                asm_output.extend(translate(&path, &mut next_instr, &mut call_counter));
            }
        }
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
