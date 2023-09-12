use std::fs::{read_to_string, write};
use symbol_table::SymbolTable;

mod instruction_parser {
    #[derive(Debug)]
    pub struct ParsedInstruction {
        pub comp: String,
        pub dest: Option<String>,
        pub jump: Option<String>,
    }

    pub fn parse_instruction(instruction: &str) -> ParsedInstruction {
        if instruction.contains("=") && instruction.contains(";") {
            let split: Vec<&str> = instruction.split(&['=', ';']).collect();
            return ParsedInstruction {
                comp: String::from(split[1]),
                dest: Some(String::from(split[0])),
                jump: Some(String::from(split[2])),
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
}

mod code_parser {
    pub fn parse_comp(comp: &str) -> String {
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

    pub fn parse_dest(dest: &str) -> String {
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

    pub fn parse_jump(jump: &str) -> String {
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
}

mod symbol_table {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct SymbolTable {
        table: HashMap<String, u16>,
        mem_counter: u16,
    }

    impl SymbolTable {
        pub fn initialize() -> Self {
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

        pub fn add_label(&mut self, name: &str, value: u16) {
            // Used in first pass for label symbols: given a label name and
            // instruction number, store it in the symbol table.
            self.table.insert(String::from(name), value);
        }

        pub fn maybe_add_and_return(&mut self, name: &str) -> u16 {
            // Used in second pass for both label and variable symbols: if the
            // symbol does not exist in the table, it is a new variable. Add it to
            // the table and increment the memory counter. In any case, return the
            // value stored in the table for the symbol name.
            if !self.table.contains_key(name) {
                self.table.insert(String::from(name), self.mem_counter);
                self.mem_counter += 1;
            }
            self.table[name]
        }
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

pub fn write_lines(outfile: &str, binary_output: &Vec<String>) {
    write(outfile, binary_output.join("\n"))
        .expect(&format!("Failed to write hack output to {}", outfile));
}

pub fn assemble(infile: &str) -> Vec<String> {
    let lines = read_lines(infile);
    let mut symbol_table = SymbolTable::initialize();
    set_label_symbols(&mut symbol_table, &lines);
    let binary_output = parse_instructions(&lines, &mut symbol_table);
    binary_output
}

fn set_label_symbols(symbol_table: &mut SymbolTable, lines: &Vec<String>) {
    // First pass: traverse the valid lines of the file, and when a label
    // definition is found, add the label to the symbol table.
    let mut counter: u16 = 0;
    for line in lines {
        if line.starts_with('(') {
            let label = line.strip_prefix('(').unwrap().strip_suffix(')').unwrap();
            symbol_table.add_label(label, counter);
        } else {
            counter += 1;
        }
    }
}

fn parse_instructions(lines: &Vec<String>, symbol_table: &mut SymbolTable) -> Vec<String> {
    let mut binary_output: Vec<String> = Vec::new();
    for line in lines {
        if line.starts_with('(') {
            continue;
        } else if line.starts_with('@') {
            binary_output.push(parse_a_instruction(line, symbol_table));
        } else {
            binary_output.push(parse_c_instruction(line));
        }
    }
    binary_output
}

fn parse_a_instruction(instruction: &str, symbol_table: &mut SymbolTable) -> String {
    let instruction = instruction.strip_prefix('@').unwrap();
    let address: u16 = match instruction.parse() {
        Ok(address) => address,
        Err(_) => symbol_table.maybe_add_and_return(instruction),
    };
    format!("{:016b}", address)
}

fn parse_c_instruction(instruction: &str) -> String {
    let parsed_instruction = instruction_parser::parse_instruction(instruction);
    let comp = code_parser::parse_comp(&parsed_instruction.comp);
    let dest = match parsed_instruction.dest {
        Some(dest) => code_parser::parse_dest(&dest),
        None => String::from("000"),
    };
    let jump = match parsed_instruction.jump {
        Some(jump) => code_parser::parse_jump(&jump),
        None => String::from("000"),
    };
    format!("111{comp}{dest}{jump}")
}
