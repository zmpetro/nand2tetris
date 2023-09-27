mod vm_translator;

use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: vm_translator_rs <infile>");
    }
    let infile = Path::new(&args[1]);
    let outfile = infile.with_extension("asm");
    println!(
        "Translating {} and writing hack assembly output to {}...",
        infile.to_str().unwrap(),
        outfile.to_str().unwrap()
    );
    let asm_output = vm_translator::translate(infile);
    vm_translator::write_lines(&outfile, &asm_output);
    println!(
        "Translation successful; output written to {}",
        outfile.to_str().unwrap()
    );
}
