use std::fs::read_to_string;
use std::path::Path;

mod tokenizer {
    pub enum Keyword {
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
        pub fn to_string(&self) -> &str {
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

    pub enum Symbol {
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
        pub fn to_string(&self) -> String {
            match self {
                Token::Keyword { keyword } => {
                    format!("<keyword> {} </keyword>", keyword.to_string())
                }
                Token::Symbol { symbol } => format!("<symbol> {} </symbol>", symbol.to_string()),
                Token::IntegerConstant { value } => {
                    format!("<integerConstant> {} </integerConstant>", value)
                }
                Token::StringConstant { literal } => {
                    format!("<stringConstant> {} </stringConstant>", literal)
                }
                Token::Identifier { literal } => format!("<identifier> {} </identifier>", literal),
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
                while self.source[self.index] != b'\n' {
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

        fn get_symbol(&mut self) -> Option<Symbol> {
            let next_char = self.source[self.index];
            let matched_symbol = match next_char {
                b'{' => Some(Symbol::LCurly),
                b'}' => Some(Symbol::RCurly),
                b'(' => Some(Symbol::LParen),
                b')' => Some(Symbol::RParen),
                b'[' => Some(Symbol::LBracket),
                b']' => Some(Symbol::RBracket),
                b'.' => Some(Symbol::Period),
                b',' => Some(Symbol::Comma),
                b';' => Some(Symbol::Semicolon),
                b'+' => Some(Symbol::Plus),
                b'-' => Some(Symbol::Minus),
                b'*' => Some(Symbol::Asterisk),
                b'/' => Some(Symbol::Slash),
                b'&' => Some(Symbol::Ampersand),
                b'|' => Some(Symbol::Pipe),
                b'<' => Some(Symbol::LessThan),
                b'>' => Some(Symbol::GreaterThan),
                b'=' => Some(Symbol::Equals),
                b'~' => Some(Symbol::Tilde),
                _ => None,
            };
            matched_symbol
        }

        fn get_keyword(&mut self) -> Option<Keyword> {
            /*
             * Keywords that must be followed by a space:
             * class, constructor, function, method, field, static, var, int,
             * char, boolean, void, let, do
             *
             * Keywords that can be following by a space or a symbol:
             * true, false, null, this, if, else, while, return
             */
            let keyword_slice = &self.source[self.index..self.index + 12];
            // 12 is the longest keyword `constructor` followed by a space
            let matched_keyword = match keyword_slice {
                kw if kw.starts_with(b"class ") => Some(Keyword::Class),
                kw if kw.starts_with(b"constructor ") => Some(Keyword::Constructor),
                kw if kw.starts_with(b"function ") => Some(Keyword::Function),
                kw if kw.starts_with(b"method ") => Some(Keyword::Method),
                kw if kw.starts_with(b"field ") => Some(Keyword::Field),
                kw if kw.starts_with(b"static ") => Some(Keyword::Static),
                kw if kw.starts_with(b"var ") => Some(Keyword::Var),
                kw if kw.starts_with(b"int ") => Some(Keyword::Int),
                kw if kw.starts_with(b"char ") => Some(Keyword::Char),
                kw if kw.starts_with(b"boolean ") => Some(Keyword::Boolean),
                kw if kw.starts_with(b"void ") => Some(Keyword::Void),
                kw if kw.starts_with(b"let ") => Some(Keyword::Let),
                kw if kw.starts_with(b"do ") => Some(Keyword::Do),
                kw if kw.starts_with(b"true")
                    && !kw[4].is_ascii_alphanumeric()
                    && kw[4] != b'_' =>
                {
                    Some(Keyword::True)
                }
                kw if kw.starts_with(b"false")
                    && !kw[5].is_ascii_alphanumeric()
                    && kw[5] != b'_' =>
                {
                    Some(Keyword::False)
                }
                kw if kw.starts_with(b"null")
                    && !kw[4].is_ascii_alphanumeric()
                    && kw[4] != b'_' =>
                {
                    Some(Keyword::Null)
                }
                kw if kw.starts_with(b"this")
                    && !kw[4].is_ascii_alphanumeric()
                    && kw[4] != b'_' =>
                {
                    Some(Keyword::This)
                }
                kw if kw.starts_with(b"if") && !kw[2].is_ascii_alphanumeric() && kw[2] != b'_' => {
                    Some(Keyword::If)
                }
                kw if kw.starts_with(b"else")
                    && !kw[4].is_ascii_alphanumeric()
                    && kw[4] != b'_' =>
                {
                    Some(Keyword::Else)
                }
                kw if kw.starts_with(b"while")
                    && !kw[5].is_ascii_alphanumeric()
                    && kw[5] != b'_' =>
                {
                    Some(Keyword::While)
                }
                kw if kw.starts_with(b"return")
                    && !kw[6].is_ascii_alphanumeric()
                    && kw[6] != b'_' =>
                {
                    Some(Keyword::Return)
                }
                _ => None,
            };
            matched_keyword
        }

        fn get_integer_constant(&mut self) -> Option<u16> {
            let mut integer: Vec<u8> = vec![];
            let mut cur_index = self.index;
            while self.source[cur_index].is_ascii_digit() {
                integer.push(self.source[cur_index]);
                cur_index += 1;
            }
            if integer.is_empty() {
                return None;
            } else {
                let integer_str = String::from_utf8(integer).unwrap();
                let parsed_integer: Result<u16, _> = integer_str.parse();
                return Some(parsed_integer.unwrap());
            }
        }

        fn get_string_constant(&mut self) -> Option<String> {
            if self.source[self.index] != b'"' {
                return None;
            }
            let mut literal: Vec<u8> = vec![];
            let mut cur_index = self.index;
            cur_index += 1; // Advance past initial double quote
            while self.source[cur_index] != b'"' {
                literal.push(self.source[cur_index]);
                cur_index += 1;
            }
            Some(String::from_utf8(literal).unwrap())
        }

        pub fn advance(&mut self) {
            self.ignore_whitespace_and_comments();
            if let Some(symbol) = self.get_symbol() {
                self.index += 1;
                self.current_token = Some(Token::Symbol { symbol: symbol });
            } else if let Some(keyword) = self.get_keyword() {
                self.index += keyword.to_string().chars().count();
                self.current_token = Some(Token::Keyword { keyword: keyword });
            } else if let Some(integer_constant) = self.get_integer_constant() {
                self.index += integer_constant.to_string().len();
                self.current_token = Some(Token::IntegerConstant {
                    value: integer_constant,
                });
            } else if let Some(string_constant) = self.get_string_constant() {
                self.index += string_constant.len() + 2; // Add 2 for quote symbols
                self.current_token = Some(Token::StringConstant {
                    literal: string_constant,
                });
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
    tokenizer.advance();
    result.push(tokenizer.current_token.as_ref().unwrap().to_string());
    tokenizer.advance();
    result.push(tokenizer.current_token.as_ref().unwrap().to_string());
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
