use std::fs::read_to_string;
use std::path::Path;

mod tokenizer {
    use std::str::Chars;

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

    pub enum Token {
        Keyword { keyword: Keyword },
        Symbol { symbol: Symbol },
        IntegerConstant { value: u16 },
        StringConstant { literal: String },
        Identifier { literal: String },
    }

    impl Token {
        pub fn to_string(&self) -> &str {
            match *self {
                Token::Keyword { .. } => "keyword",
                Token::Symbol { .. } => "symbol",
                Token::IntegerConstant { .. } => "integerConstant",
                Token::StringConstant { .. } => "stringConstant",
                Token::Identifier { .. } => "identifier",
            }
        }
    }

    pub struct Tokenizer {
        pub source: Vec<u8>, // Jack source code
        pub index: usize,    // Position in source
        pub current_token: Option<Token>, // current_token is None when Tokenizer
                             // is created and when there are no
                             // tokens remaining, otherwise it is Some
    }

    impl Tokenizer {
        pub fn new(source: Vec<u8>) -> Self {
            Self {
                source: source,
                index: 0,
                current_token: None,
            }
        }

        fn ignore_whitespace(&mut self) {
            while self.source[self.index].is_ascii_whitespace() {
                self.index += 1;
            }
        }

        fn ignore_singleline_comment(&mut self) {
            if &self.source[self.index..self.index + 2] == b"//" {
                self.index += 2;
                while &self.source[self.index..self.index + 1] != b"\n" {
                    self.index += 1;
                }
                self.ignore_whitespace();
            }
        }

        fn ignore_multiline_comment(&mut self) {
            if &self.source[self.index..self.index + 3] == b"/**" {
                self.index += 3;
                while &self.source[self.index..self.index + 2] != b"*/" {
                    self.index += 1;
                }
                self.index += 2;
                self.ignore_whitespace();
            }
        }

        fn ignore_whitespace_and_comments(&mut self) {
            loop {
                let starting_index = self.index;
                self.ignore_whitespace();
                self.ignore_singleline_comment();
                self.ignore_multiline_comment();
                if self.index == starting_index {
                    break;
                }
            }
        }

        fn get_symbol(&mut self) -> Option<Token> {
            let symbol_slice = &self.source[self.index..self.index + 1];
            let matched_symbol = match symbol_slice {
                b"{" => Some(Token::Symbol {
                    symbol: Symbol::LCurly,
                }),
                b"}" => Some(Token::Symbol {
                    symbol: Symbol::RCurly,
                }),
                b"(" => Some(Token::Symbol {
                    symbol: Symbol::LParen,
                }),
                b")" => Some(Token::Symbol {
                    symbol: Symbol::RParen,
                }),
                b"[" => Some(Token::Symbol {
                    symbol: Symbol::LBracket,
                }),
                b"]" => Some(Token::Symbol {
                    symbol: Symbol::RBracket,
                }),
                b"." => Some(Token::Symbol {
                    symbol: Symbol::Period,
                }),
                b"," => Some(Token::Symbol {
                    symbol: Symbol::Comma,
                }),
                b";" => Some(Token::Symbol {
                    symbol: Symbol::Semicolon,
                }),
                b"+" => Some(Token::Symbol {
                    symbol: Symbol::Plus,
                }),
                b"-" => Some(Token::Symbol {
                    symbol: Symbol::Minus,
                }),
                b"*" => Some(Token::Symbol {
                    symbol: Symbol::Asterisk,
                }),
                b"/" => Some(Token::Symbol {
                    symbol: Symbol::Slash,
                }),
                b"&" => Some(Token::Symbol {
                    symbol: Symbol::Ampersand,
                }),
                b"|" => Some(Token::Symbol {
                    symbol: Symbol::Pipe,
                }),
                b"<" => Some(Token::Symbol {
                    symbol: Symbol::LessThan,
                }),
                b">" => Some(Token::Symbol {
                    symbol: Symbol::GreaterThan,
                }),
                b"=" => Some(Token::Symbol {
                    symbol: Symbol::Equals,
                }),
                b"~" => Some(Token::Symbol {
                    symbol: Symbol::Tilde,
                }),
                _ => None,
            };
            matched_symbol
        }

        pub fn advance(&mut self) {
            self.ignore_whitespace_and_comments();
            if let Some(symbol) = self.get_symbol() {
                self.current_token = Some(symbol);
                self.index += 1;
            }
        }
    }
}

fn read_infile(infile: &Path) -> String {
    read_to_string(infile).unwrap().parse().unwrap()
}

pub fn analyze_file(infile: &Path) -> Vec<String> {
    let source = read_infile(infile).into_bytes();
    let mut tokenizer = tokenizer::Tokenizer::new(source);

    let mut result = vec![];
    result.push(String::from("<tokens>"));
    println!(
        "index: {}  char: {}",
        tokenizer.index, tokenizer.source[tokenizer.index] as char
    );
    tokenizer.advance();
    println!(
        "index: {}  char: {}",
        tokenizer.index, tokenizer.source[tokenizer.index] as char
    );
    println!(
        "current token: {}",
        tokenizer.current_token.unwrap().to_string()
    );
    result.push(String::from("</tokens>\n"));
    result
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
