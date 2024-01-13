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
            panic!("Failed to write xml output: {e}");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: syntax_analyzer_rs <infile or directory>");
    }
    let infile_or_directory = Path::new(&args[1]);
    let mut files_to_analyze: Vec<PathBuf> = vec![];
    if infile_or_directory.is_dir() {
        for entry in infile_or_directory.read_dir().unwrap() {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().unwrap() == "jack" {
                    files_to_analyze.push(path);
                }
            }
        }
    } else {
        files_to_analyze.push(infile_or_directory.to_path_buf());
    };
    for infile in files_to_analyze {
        let outfile = infile.with_extension("xml");
        println!(
            "Analzying {} and writing xml output to {} ...",
            infile.to_str().unwrap(),
            outfile.to_str().unwrap()
        );
        let xml_output = syntax_analyzer::analyze_file(&infile);
        write_lines(&outfile, &xml_output);
        println!(
            "Analysis successful; output written to {}",
            outfile.to_str().unwrap()
        );
    }
}
