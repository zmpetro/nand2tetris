mod vm_translator;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: vm_translator_rs <infile>");
    }
    let infile = &args[1];
    let outfile: Vec<&str> = infile.split(".").collect();
    let outfile = format!("{}.asm", outfile[0]);
    println!(
        "Translating {} and writing hack assembly output to {}...",
        infile, outfile
    );
    let asm_output = vm_translator::translate(infile);
    vm_translator::write_lines(&outfile, &asm_output);
    println!("Translation successful; output written to {}", outfile);
}
