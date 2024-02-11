#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Keyword { keyword: Keyword },
    Symbol { symbol: Symbol },
    IntegerConstant { value: usize },
    StringConstant { literal: String },
    Identifier { literal: String },
}

// Used to return a Keyword or Identifier from a function
pub enum KeywordOrIdentifier {
    Kw { keyword: Keyword },
    Id { identifier: String },
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
        while let Some(&char) = self.source.get(self.index) {
            if char.is_ascii_whitespace() {
                self.index += 1;
            } else {
                break;
            }
        }
    }

    fn ignore_singleline_comment(&mut self) {
        if let Some(slice) = self.source.get(self.index..self.index + 2) {
            if slice == b"//" {
                self.index += 2;
                while let Some(&char) = self.source.get(self.index) {
                    if char != b'\n' {
                        self.index += 1;
                    } else {
                        break;
                    }
                }
                self.ignore_whitespace();
            }
        }
    }

    fn ignore_multiline_comment(&mut self) {
        if let Some(start_slice) = self.source.get(self.index..self.index + 3) {
            if start_slice == b"/**" {
                self.index += 3;
                while let Some(end_slice) = self.source.get(self.index..self.index + 2) {
                    if end_slice != b"*/" {
                        self.index += 1;
                    } else {
                        break;
                    }
                }
                self.index += 2;
                self.ignore_whitespace();
            }
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

    fn get_symbol(&self) -> Option<Symbol> {
        if let Some(&char) = self.source.get(self.index) {
            let matched_symbol = match char {
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
            return matched_symbol;
        } else {
            return None;
        }
    }

    fn get_integer_constant(&self) -> Option<usize> {
        let mut integer: Vec<u8> = vec![];
        let mut cur_index = self.index;
        while let Some(&char) = self.source.get(cur_index) {
            if char.is_ascii_digit() {
                integer.push(char);
                cur_index += 1;
            } else {
                break;
            }
        }
        if integer.is_empty() {
            return None;
        } else {
            let integer_str = String::from_utf8(integer).unwrap();
            let parsed_integer: Result<usize, _> = integer_str.parse();
            return Some(parsed_integer.unwrap());
        }
    }

    fn get_string_constant(&self) -> Option<String> {
        if let Some(&char) = self.source.get(self.index) {
            if char != b'"' {
                return None;
            }
        } else {
            return None;
        }
        let mut literal: Vec<u8> = vec![];
        let mut cur_index = self.index;
        cur_index += 1; // Advance past initial double quote
        while let Some(&char) = self.source.get(cur_index) {
            if char != b'"' {
                literal.push(char);
                cur_index += 1;
            } else {
                break;
            }
        }
        Some(String::from_utf8(literal).unwrap())
    }

    fn get_keyword(&self, kw_or_id: &str) -> Option<Keyword> {
        let keyword = match kw_or_id {
            "class" => Some(Keyword::Class),
            "constructor" => Some(Keyword::Constructor),
            "function" => Some(Keyword::Function),
            "method" => Some(Keyword::Method),
            "field" => Some(Keyword::Field),
            "static" => Some(Keyword::Static),
            "var" => Some(Keyword::Var),
            "int" => Some(Keyword::Int),
            "char" => Some(Keyword::Char),
            "boolean" => Some(Keyword::Boolean),
            "void" => Some(Keyword::Void),
            "true" => Some(Keyword::True),
            "false" => Some(Keyword::False),
            "null" => Some(Keyword::Null),
            "this" => Some(Keyword::This),
            "let" => Some(Keyword::Let),
            "do" => Some(Keyword::Do),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "while" => Some(Keyword::While),
            "return" => Some(Keyword::Return),
            _ => None,
        };
        keyword
    }

    fn get_keyword_or_identifier(&self) -> Option<KeywordOrIdentifier> {
        if let Some(&char) = self.source.get(self.index) {
            if !char.is_ascii_alphabetic() && char != b'_' {
                return None;
            }
        } else {
            return None;
        }
        let mut kw_or_id: Vec<u8> = vec![];
        let mut cur_index = self.index;
        while let Some(&char) = self.source.get(cur_index) {
            if char.is_ascii_alphanumeric() || char == b'_' {
                kw_or_id.push(char);
                cur_index += 1;
            } else {
                break;
            }
        }
        let kw_or_id = String::from_utf8(kw_or_id).unwrap();
        if let Some(keyword) = self.get_keyword(&kw_or_id) {
            return Some(KeywordOrIdentifier::Kw { keyword: keyword });
        } else {
            return Some(KeywordOrIdentifier::Id {
                identifier: kw_or_id,
            });
        }
    }

    pub fn advance(&mut self) {
        self.ignore_whitespace_and_comments();
        if let Some(symbol) = self.get_symbol() {
            self.index += 1;
            self.current_token = Some(Token::Symbol { symbol: symbol });
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
        } else if let Some(kw_or_id) = self.get_keyword_or_identifier() {
            match kw_or_id {
                KeywordOrIdentifier::Kw { keyword } => {
                    self.index += keyword.to_string().chars().count();
                    self.current_token = Some(Token::Keyword { keyword: keyword });
                }
                KeywordOrIdentifier::Id { identifier } => {
                    self.index += identifier.len();
                    self.current_token = Some(Token::Identifier {
                        literal: identifier,
                    });
                }
            };
        } else {
            self.current_token = None;
        }
    }
}
