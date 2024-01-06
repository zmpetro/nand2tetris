use std::fs::read_to_string;
use std::path::Path;

mod tokenizer {
    enum Keyword {
        Class,
        Constructor,
        Function,
        Method,
        Field,
        Static,
        Var,
        Int,
        Char,
        Boolean,
        Void,
        True,
        False,
        Null,
        This,
        Let,
        Do,
        If,
        Else,
        While,
        Return,
    }

    impl Keyword {
        fn to_string(&self) -> &str {
            match *self {
                Keyword::Class => "class",
                Keyword::Constructor => "constructor",
                Keyword::Function => "function",
                Keyword::Method => "method",
                Keyword::Field => "field",
                Keyword::Static => "static",
                Keyword::Var => "var",
                Keyword::Int => "int",
                Keyword::Char => "char",
                Keyword::Boolean => "boolean",
                Keyword::Void => "void",
                Keyword::True => "true",
                Keyword::False => "false",
                Keyword::Null => "null",
                Keyword::This => "this",
                Keyword::Let => "let",
                Keyword::Do => "do",
                Keyword::If => "if",
                Keyword::Else => "else",
                Keyword::While => "while",
                Keyword::Return => "return",
            }
        }
    }

    enum Symbol {
        LCurly,
        RCurly,
        LParen,
        RParen,
        LBracket,
        RBracket,
        Period,
        Comma,
        Semicolon,
        Plus,
        Minus,
        Asterisk,
        Slash,
        Ampersand,
        Pipe,
        LessThan,
        GreaterThan,
        Equals,
        Tilde,
    }

    impl Symbol {
        fn to_string(&self) -> &str {
            match *self {
                Symbol::LCurly => "{",
                Symbol::RCurly => "}",
                Symbol::LParen => "(",
                Symbol::RParen => ")",
                Symbol::LBracket => "[",
                Symbol::RBracket => "]",
                Symbol::Period => ".",
                Symbol::Comma => ",",
                Symbol::Semicolon => ";",
                Symbol::Plus => "+",
                Symbol::Minus => "-",
                Symbol::Asterisk => "*",
                Symbol::Slash => "/",
                Symbol::Ampersand => "&amp;",
                Symbol::Pipe => "|",
                Symbol::LessThan => "&lt;",
                Symbol::GreaterThan => "&gt;",
                Symbol::Equals => "=",
                Symbol::Tilde => "~",
            }
        }
    }

    enum Token {
        Keyword { keyword: Keyword },
        Symbol { symbol: Symbol },
        IntegerConstant { value: u16 },
        StringConstant { literal: String },
        Identifier { literal: String },
    }

    impl Token {
        fn to_string(&self) -> &str {
            match *self {
                Token::Keyword { .. } => "keyword",
                Token::Symbol { .. } => "symbol",
                Token::IntegerConstant { .. } => "keyword",
                Token::StringConstant { .. } => "stringConstant",
                Token::Identifier { .. } => "integerConstant",
            }
        }
    }

    struct Tokenizer {
        current_token: Token,
    }

    impl Tokenizer {
        fn advance(&mut self) {}
    }
}

fn read_infile(infile: &Path) -> String {
    read_to_string(infile).unwrap().parse().unwrap()
}

pub fn analyze_file(infile: &Path) -> Vec<String> {
    let source = read_infile(infile);
    vec![]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_nothing() {
        let x = 5;
        let y = 5;
        assert_eq!(x, y);
    }
}
