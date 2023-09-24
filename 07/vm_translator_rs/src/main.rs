mod vm_translator;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Usage: vm_translator_rs <infile> <outfile>");
    }
    let infile = &args[1];
    let outfile = &args[2];
    let asm_output = vm_translator::translate(infile);
    println!(
        "Translating {} and writing hack assembly output to {}...",
        infile, outfile
    );
    vm_translator::write_lines(outfile, &asm_output);
    println!("Translation successful; output written to {}", outfile);
}
