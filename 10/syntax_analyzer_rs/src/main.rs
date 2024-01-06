mod syntax_analyzer;

use std::env;
use std::fs::write;
use std::path::{Path, PathBuf};

fn write_lines(outfile: &PathBuf, xml_output: &[String]) {
    write(outfile, xml_output.join("\n")).expect(&format!(
        "Failed to write jack xml output to {}",
        outfile.to_str().unwrap()
    ));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: syntax_analyzer_rs <infile>");
    }
    let infile = Path::new(&args[1]);
    let outfile = infile.with_extension("xml");
    println!(
        "Analzying {} and writing jack xml output to {} ...",
        infile.to_str().unwrap(),
        outfile.to_str().unwrap()
    );
    let xml_output = syntax_analyzer::analyze_file(infile);
    write_lines(&outfile, &xml_output);
    println!(
        "Analysis successful; output written to {}",
        outfile.to_str().unwrap()
    );
}
