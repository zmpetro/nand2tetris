mod assembler;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Usage: assembler_rs <infile> <outfile>");
    }
    let infile = &args[1];
    let outfile = &args[2];
    let binary_output = assembler::assemble(infile);
    println!(
        "Assembling {} and writing hack output to {}...",
        infile, outfile
    );
    assembler::write_lines(outfile, &binary_output);
    println!("Assembly successful; output written to {}", outfile);
}
