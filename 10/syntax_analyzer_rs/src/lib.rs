mod compilation_engine;
mod tokenizer;

use std::fs::read_to_string;
use std::fs::File;
use std::path::{Path, PathBuf};

use xml::writer::{EmitterConfig, XmlEvent};

pub fn read_infile(infile: &Path) -> String {
    read_to_string(infile).unwrap().parse().unwrap()
}

pub fn analyze_file(infile: &Path) -> Vec<String> {
    let source = read_infile(infile).into_bytes();
    let tokenizer = tokenizer::Tokenizer::new(source);
    let mut compilation_engine = compilation_engine::CompilationEngine::new(tokenizer);
    compilation_engine
        .compile_class()
        .expect("Compilation failed");
    compilation_engine.result
}

pub fn make_event_from_line(line: &str) -> XmlEvent {
    if let Some(name) = line.strip_prefix("+") {
        XmlEvent::start_element(name).into()
    } else if line.starts_with("-") {
        XmlEvent::end_element().into()
    } else {
        XmlEvent::characters(line).into()
    }
}

pub fn write_lines(outfile: &PathBuf, xml_output: &[String]) {
    let outfile = File::create(outfile).unwrap();
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .write_document_declaration(false)
        .normalize_empty_elements(false)
        .create_writer(outfile);
    for line in xml_output {
        let event = make_event_from_line(line);
        if let Err(e) = writer.write(event) {
            panic!("Failed to write xml output: {e}");
        }
    }
}
