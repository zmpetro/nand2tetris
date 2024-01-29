mod compilation_engine;
mod symbol_table;
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

pub fn get_emitter_config() -> EmitterConfig {
    EmitterConfig {
        line_separator: "\n".into(),
        indent_string: "  ".into(),
        perform_indent: true,
        perform_escaping: false,
        write_document_declaration: false,
        normalize_empty_elements: false,
        cdata_to_characters: false,
        keep_element_names_stack: true,
        autopad_comments: true,
        pad_self_closing: true,
    }
}

pub fn write_lines(outfile: &PathBuf, xml_output: &[String]) {
    let outfile = File::create(outfile).unwrap();
    let emitter_config = get_emitter_config();
    let mut writer = emitter_config.create_writer(outfile);
    for line in xml_output {
        let event = make_event_from_line(line);
        if let Err(e) = writer.write(event) {
            panic!("Failed to write xml output: {e}");
        }
    }
}
