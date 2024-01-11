mod syntax_analyzer;

use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};

use xml::writer::{EmitterConfig, XmlEvent};

fn make_event_from_line(line: &str) -> XmlEvent {
    if let Some(name) = line.strip_prefix("+") {
        XmlEvent::start_element(name).into()
    } else if line.starts_with("-") {
        XmlEvent::end_element().into()
    } else {
        XmlEvent::characters(line).into()
    }
}

fn write_lines(outfile: &PathBuf, xml_output: &[String]) {
    let outfile = File::create(outfile).unwrap();
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .write_document_declaration(false)
        .normalize_empty_elements(false)
        .create_writer(outfile);
    for line in xml_output {
        let event = make_event_from_line(line);
        if let Err(e) = writer.write(event) {
            panic!("Failed to write jack xml output: {e}");
        }
    }
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
