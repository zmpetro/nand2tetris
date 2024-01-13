mod compilation_engine;
mod tokenizer;

use std::env;
use std::fs::read_to_string;
use std::fs::File;
use std::path::{Path, PathBuf};

use xml::writer::{EmitterConfig, XmlEvent};

fn read_infile(infile: &Path) -> String {
    read_to_string(infile).unwrap().parse().unwrap()
}

fn analyze_file(infile: &Path) -> Vec<String> {
    let source = read_infile(infile).into_bytes();
    let tokenizer = tokenizer::Tokenizer::new(source);
    let mut compilation_engine = compilation_engine::CompilationEngine::new(tokenizer);
    compilation_engine
        .compile_class()
        .expect("Compilation failed");
    compilation_engine.result
}
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
        let xml_output = analyze_file(&infile);
        write_lines(&outfile, &xml_output);
        println!(
            "Analysis successful; output written to {}",
            outfile.to_str().unwrap()
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::{analyze_file, make_event_from_line, read_infile};

    use std::env;
    use std::io::Cursor;
    use std::path::PathBuf;

    use xml::writer::EmitterConfig;

    fn write_lines(buff: &mut Cursor<Vec<u8>>, xml_output: &[String]) {
        let mut writer = EmitterConfig::new()
            .perform_indent(true)
            .write_document_declaration(false)
            .normalize_empty_elements(false)
            .create_writer(buff);
        for line in xml_output {
            let event = make_event_from_line(line);
            if let Err(e) = writer.write(event) {
                panic!("Failed to write xml output: {e}");
            }
        }
    }

    fn generate_xml_and_compare(jack_infile: &PathBuf, xml_comparison_file: &PathBuf) {
        let xml_comparison = read_infile(&xml_comparison_file);
        let analysis_result = analyze_file(&jack_infile);
        let mut buff: Cursor<Vec<u8>> = Cursor::new(vec![]);
        write_lines(&mut buff, &analysis_result);
        let xml_output = String::from_utf8(buff.into_inner()).unwrap();
        assert_eq!(xml_comparison, xml_output);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ExpressionLessSquare_Main() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("test_data/ExpressionLessSquare/Main.jack");
        let xml_comparison_file = current_dir.join("test_data/ExpressionLessSquare/Main.xml");
        generate_xml_and_compare(&jack_infile, &xml_comparison_file);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ExpressionLessSquare_Square() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("test_data/ExpressionLessSquare/Square.jack");
        let xml_comparison_file = current_dir.join("test_data/ExpressionLessSquare/Square.xml");
        generate_xml_and_compare(&jack_infile, &xml_comparison_file);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ExpressionLessSquare_SquareGame() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("test_data/ExpressionLessSquare/SquareGame.jack");
        let xml_comparison_file = current_dir.join("test_data/ExpressionLessSquare/SquareGame.xml");
        generate_xml_and_compare(&jack_infile, &xml_comparison_file);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_Square_Main() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("test_data/Square/Main.jack");
        let xml_comparison_file = current_dir.join("test_data/Square/Main.xml");
        generate_xml_and_compare(&jack_infile, &xml_comparison_file);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_Square_Square() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("test_data/Square/Square.jack");
        let xml_comparison_file = current_dir.join("test_data/Square/Square.xml");
        generate_xml_and_compare(&jack_infile, &xml_comparison_file);
    }
    #[test]
    #[allow(non_snake_case)]
    fn test_Square_SquareGame() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("test_data/Square/SquareGame.jack");
        let xml_comparison_file = current_dir.join("test_data/Square/SquareGame.xml");
        generate_xml_and_compare(&jack_infile, &xml_comparison_file);
    }
}
