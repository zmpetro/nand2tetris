use syntax_analyzer_rs::{analyze_file, write_lines};

use std::env;
use std::path::{Path, PathBuf};

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
