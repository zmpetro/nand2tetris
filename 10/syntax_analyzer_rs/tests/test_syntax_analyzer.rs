use syntax_analyzer_rs::{analyze_file, get_emitter_config, make_event_from_line, read_infile};

mod tests {
    use crate::{analyze_file, get_emitter_config, make_event_from_line, read_infile};

    use std::env;
    use std::io::Cursor;
    use std::path::PathBuf;

    fn write_lines(buff: &mut Cursor<Vec<u8>>, xml_output: &[String]) {
        let emitter_config = get_emitter_config();
        let mut writer = emitter_config.create_writer(buff);
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
        let jack_infile = current_dir.join("tests/test_data/ExpressionLessSquare/Main.jack");
        let xml_comparison_file = current_dir.join("tests/test_data/ExpressionLessSquare/Main.xml");
        generate_xml_and_compare(&jack_infile, &xml_comparison_file);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ExpressionLessSquare_Square() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("tests/test_data/ExpressionLessSquare/Square.jack");
        let xml_comparison_file =
            current_dir.join("tests/test_data/ExpressionLessSquare/Square.xml");
        generate_xml_and_compare(&jack_infile, &xml_comparison_file);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ExpressionLessSquare_SquareGame() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("tests/test_data/ExpressionLessSquare/SquareGame.jack");
        let xml_comparison_file =
            current_dir.join("tests/test_data/ExpressionLessSquare/SquareGame.xml");
        generate_xml_and_compare(&jack_infile, &xml_comparison_file);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_Square_Main() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("tests/test_data/Square/Main.jack");
        let xml_comparison_file = current_dir.join("tests/test_data/Square/Main.xml");
        generate_xml_and_compare(&jack_infile, &xml_comparison_file);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_Square_Square() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("tests/test_data/Square/Square.jack");
        let xml_comparison_file = current_dir.join("tests/test_data/Square/Square.xml");
        generate_xml_and_compare(&jack_infile, &xml_comparison_file);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_Square_SquareGame() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("tests/test_data/Square/SquareGame.jack");
        let xml_comparison_file = current_dir.join("tests/test_data/Square/SquareGame.xml");
        generate_xml_and_compare(&jack_infile, &xml_comparison_file);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ArrayTest_Main() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("tests/test_data/ArrayTest/Main.jack");
        let xml_comparison_file = current_dir.join("tests/test_data/ArrayTest/Main.xml");
        generate_xml_and_compare(&jack_infile, &xml_comparison_file);
    }
}
