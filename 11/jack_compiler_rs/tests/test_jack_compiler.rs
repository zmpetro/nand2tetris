use jack_compiler_rs::{compile_file, read_infile};

mod tests {
    use crate::{compile_file, read_infile};

    use std::env;
    use std::path::PathBuf;

    use pretty_assertions::assert_eq;

    fn compile_jack_and_compare(jack_infile: &PathBuf, vm_comparison_file: &PathBuf) {
        let vm_comparison = read_infile(&vm_comparison_file);
        let mut compilation_result = compile_file(&jack_infile).join("\n");
        compilation_result.push('\n');
        assert_eq!(vm_comparison, compilation_result);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_Average_Main() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("tests/test_data/Average/Main.jack");
        let vm_comparison_file = current_dir.join("tests/test_data/Average/Main.vm");
        compile_jack_and_compare(&jack_infile, &vm_comparison_file);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ComplexArrays_Main() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("tests/test_data/ComplexArrays/Main.jack");
        let vm_comparison_file = current_dir.join("tests/test_data/ComplexArrays/Main.vm");
        compile_jack_and_compare(&jack_infile, &vm_comparison_file);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ConvertToBin_Main() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("tests/test_data/ConvertToBin/Main.jack");
        let vm_comparison_file = current_dir.join("tests/test_data/ConvertToBin/Main.vm");
        compile_jack_and_compare(&jack_infile, &vm_comparison_file);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_Pong_Ball() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("tests/test_data/Pong/Ball.jack");
        let vm_comparison_file = current_dir.join("tests/test_data/Pong/Ball.vm");
        compile_jack_and_compare(&jack_infile, &vm_comparison_file);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_Pong_Bat() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("tests/test_data/Pong/Bat.jack");
        let vm_comparison_file = current_dir.join("tests/test_data/Pong/Bat.vm");
        compile_jack_and_compare(&jack_infile, &vm_comparison_file);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_Pong_Main() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("tests/test_data/Pong/Main.jack");
        let vm_comparison_file = current_dir.join("tests/test_data/Pong/Main.vm");
        compile_jack_and_compare(&jack_infile, &vm_comparison_file);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_Pong_PongGame() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("tests/test_data/Pong/PongGame.jack");
        let vm_comparison_file = current_dir.join("tests/test_data/Pong/PongGame.vm");
        compile_jack_and_compare(&jack_infile, &vm_comparison_file);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_Seven_Main() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("tests/test_data/Seven/Main.jack");
        let vm_comparison_file = current_dir.join("tests/test_data/Seven/Main.vm");
        compile_jack_and_compare(&jack_infile, &vm_comparison_file);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_Square_Main() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("tests/test_data/Square/Main.jack");
        let vm_comparison_file = current_dir.join("tests/test_data/Square/Main.vm");
        compile_jack_and_compare(&jack_infile, &vm_comparison_file);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_Square_SquareGame() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("tests/test_data/Square/SquareGame.jack");
        let vm_comparison_file = current_dir.join("tests/test_data/Square/SquareGame.vm");
        compile_jack_and_compare(&jack_infile, &vm_comparison_file);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_Square_Square() {
        let current_dir = env::current_dir().unwrap();
        let jack_infile = current_dir.join("tests/test_data/Square/Square.jack");
        let vm_comparison_file = current_dir.join("tests/test_data/Square/Square.vm");
        compile_jack_and_compare(&jack_infile, &vm_comparison_file);
    }
}
