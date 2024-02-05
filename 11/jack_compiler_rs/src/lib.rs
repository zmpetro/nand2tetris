mod compilation_engine;
mod symbol_table;
mod tokenizer;
mod vm_writer;

use std::fs::read_to_string;
use std::fs::write;
use std::path::{Path, PathBuf};

pub fn read_infile(infile: &Path) -> String {
    read_to_string(infile).unwrap().parse().unwrap()
}

pub fn compile_file(infile: &Path) -> Vec<String> {
    let source = read_infile(infile).into_bytes();
    let tokenizer = tokenizer::Tokenizer::new(source);
    let mut compilation_engine = compilation_engine::CompilationEngine::new(tokenizer);
    compilation_engine
        .compile_class()
        .expect("Compilation failed");
    compilation_engine.vm_writer.result
}

pub fn write_lines(outfile: &PathBuf, vm_output: &[String]) {
    let mut output_to_write = vm_output.join("\n");
    output_to_write.push('\n');
    write(outfile, output_to_write).expect(&format!(
        "Failed to write VM code output to {}",
        outfile.to_str().unwrap()
    ));
}
