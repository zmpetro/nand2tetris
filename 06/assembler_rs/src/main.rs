use std::collections::HashMap;

#[derive(Debug)]
struct ParsedInstruction {
    comp: String,
    dest: Option<String>,
    jump: Option<String>,
}

fn parse_instruction(instruction: &str) -> ParsedInstruction {
    if instruction.contains("=") && instruction.contains(";") {
        let split1: Vec<&str> = instruction.split("=").collect();
        let split2: Vec<&str> = split1[1].split(";").collect();
        return ParsedInstruction {
            comp: String::from(split2[0]),
            dest: Some(String::from(split1[0])),
            jump: Some(String::from(split2[1])),
        };
    } else if instruction.contains("=") && !instruction.contains(";") {
        let split: Vec<&str> = instruction.split("=").collect();
        return ParsedInstruction {
            comp: String::from(split[1]),
            dest: Some(String::from(split[0])),
            jump: None,
        };
    } else if !instruction.contains("=") && instruction.contains(";") {
        let split: Vec<&str> = instruction.split(";").collect();
        return ParsedInstruction {
            comp: String::from(split[0]),
            dest: None,
            jump: Some(String::from(split[1])),
        };
    } else {
        panic!("Instruction could not be parsed: {}", instruction);
    }
}

fn parse_comp(comp: &str) -> String {
    let parsed_comp = match comp {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "M" => "1110000",
        "!D" => "0001101",
        "!A" => "0110001",
        "!M" => "1110001",
        "-D" => "0001111",
        "-A" => "0110011",
        "-M" => "1110011",
        "D+1" => "0011111",
        "A+1" => "0110111",
        "M+1" => "1110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "M-1" => "1110010",
        "D+A" => "0000010",
        "D+M" => "1000010",
        "D-A" => "0010011",
        "D-M" => "1010011",
        "A-D" => "0000111",
        "M-D" => "1000111",
        "D&A" => "0000000",
        "D&M" => "1000000",
        "D|A" => "0010101",
        "D|M" => "1010101",
        _ => panic!("Couldn't parse comp instruction: {}", comp),
    };
    String::from(parsed_comp)
}

fn parse_dest(dest: &str) -> String {
    let parsed_dest = match dest {
        "M" => "001",
        "D" => "010",
        "MD" => "011",
        "A" => "100",
        "AM" => "101",
        "AD" => "110",
        "AMD" => "111",
        _ => panic!("Couldn't parse dest instruction: {}", dest),
    };
    String::from(parsed_dest)
}

fn parse_jump(jump: &str) -> String {
    let parsed_jump = match jump {
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        _ => panic!("Couldn't parse jump instruction: {}", jump),
    };
    String::from(parsed_jump)
}

#[derive(Debug)]
struct SymbolTable {
    table: HashMap<String, u16>,
    mem_counter: u16,
}

impl SymbolTable {
    fn initialize() -> Self {
        SymbolTable {
            table: HashMap::from([
                (String::from("SCREEN"), 16384),
                (String::from("KBD"), 24576),
                (String::from("SP"), 0),
                (String::from("LCL"), 1),
                (String::from("ARG"), 2),
                (String::from("THIS"), 3),
                (String::from("THAT"), 4),
                (String::from("R0"), 0),
                (String::from("R1"), 1),
                (String::from("R2"), 2),
                (String::from("R3"), 3),
                (String::from("R4"), 4),
                (String::from("R5"), 5),
                (String::from("R6"), 6),
                (String::from("R7"), 7),
                (String::from("R8"), 8),
                (String::from("R9"), 9),
                (String::from("R10"), 10),
                (String::from("R11"), 11),
                (String::from("R12"), 12),
                (String::from("R13"), 13),
                (String::from("R14"), 14),
                (String::from("R15"), 15),
            ]),
            mem_counter: 16,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut symbol_table = SymbolTable::initialize();
    println!("{:?}", symbol_table);
}
