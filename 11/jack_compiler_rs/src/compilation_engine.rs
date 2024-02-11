use crate::symbol_table::{Entry, Kind, SymbolTable};
use crate::tokenizer::{Keyword, Symbol, Token, Tokenizer};
use crate::vm_writer::{MathInstr, MemorySegment, VMWriter};

const ALLOC_FN: &str = "Memory.alloc";
const MULTIPLY_FN: &str = "Math.multiply";
const DIVIDE_FN: &str = "Math.divide";
const STRING_NEW_FN: &str = "String.new";
const STRING_APPEND_FN: &str = "String.appendChar";

fn keyword_to_kind(token: &Token) -> Result<Kind, String> {
    // Utility function to convert a Token Keyword of type Static, Field, or
    // Var to its respective Kind
    match token {
        Token::Keyword { keyword } => match keyword {
            Keyword::Static => Ok(Kind::Static),
            Keyword::Field => Ok(Kind::Field),
            Keyword::Var => Ok(Kind::Var),
            _ => Err(format!("Failed to convert Keyword {:?} to Kind", keyword)),
        },
        _ => Err(format!("Failed to convert Token {:?} to Kind", token)),
    }
}

fn type_to_string(token: &Token) -> Result<String, String> {
    // Utility function to convert a Token of type Keyword:Int, Keyword:Char,
    // Keyword:Boolean, or Identifier to its corresponding String.
    match token {
        Token::Keyword { keyword } => match keyword {
            Keyword::Int | Keyword::Char | Keyword::Boolean => Ok(keyword.to_string().to_owned()),
            _ => Err(format!("Failed to convert Keyword {:?} to String", keyword)),
        },
        Token::Identifier { literal } => Ok(literal.to_owned()),
        _ => Err(format!("Failed to convert Token {:?} to String", token)),
    }
}

fn kind_to_segment(kind: &Kind) -> MemorySegment {
    // Utility function to convert a Kind to a MemorySegment.
    match kind {
        Kind::Static => MemorySegment::Static,
        Kind::Field => MemorySegment::This,
        Kind::Arg => MemorySegment::Argument,
        Kind::Var => MemorySegment::Local,
    }
}

pub struct CompilationEngine {
    symbol_table: SymbolTable,
    tokenizer: Tokenizer,
    pub vm_writer: VMWriter,
    class_name: String,
    num_fields: usize,
    if_statement_idx: usize,
    while_statement_idx: usize,
}

impl CompilationEngine {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            tokenizer: tokenizer,
            vm_writer: VMWriter::new(),
            class_name: String::from(""),
            num_fields: 0,
            if_statement_idx: 0,
            while_statement_idx: 0,
        }
    }

    fn eat_integer(&mut self) -> Result<Token, String> {
        let current_token = self.tokenizer.current_token.as_ref().unwrap().clone();
        if let Token::IntegerConstant { .. } = current_token {
            self.tokenizer.advance();
            return Ok(current_token);
        } else {
            return Err(format!(
                "Token is not an IntegerConstant: {:?}",
                current_token
            ));
        }
    }

    fn eat_string(&mut self) -> Result<Token, String> {
        let current_token = self.tokenizer.current_token.as_ref().unwrap().clone();
        if let Token::StringConstant { .. } = current_token {
            self.tokenizer.advance();
            return Ok(current_token);
        } else {
            return Err(format!(
                "Token is not an StringConstant: {:?}",
                current_token
            ));
        }
    }

    fn eat_keyword_or_symbol(
        &mut self,
        expected_tokens: Vec<Token>,
        predefined_token: Option<&Token>,
    ) -> Result<Token, String> {
        let current_token = match predefined_token {
            Some(token) => token,
            None => self.tokenizer.current_token.as_ref().unwrap(),
        }
        .clone();
        if expected_tokens.contains(&current_token) {
            if predefined_token.is_none() {
                self.tokenizer.advance();
            }
            return Ok(current_token);
        } else {
            return Err(format!(
                "current_token: {:?}  expected_token: {:?}",
                current_token, expected_tokens
            ));
        }
    }

    fn eat_variable_definition_identifier(
        &mut self,
        category: Kind,
        type_: String,
    ) -> Result<(), String> {
        let current_token = self.tokenizer.current_token.as_ref().unwrap();
        match current_token {
            Token::Identifier { literal } => {
                if let Kind::Field = category {
                    self.num_fields += 1;
                }
                self.symbol_table
                    .define(literal.to_owned(), type_, category);
                self.tokenizer.advance();
                Ok(())
            }
            _ => Err(format!("Token is not an Identifier: {:?}", current_token)),
        }
    }

    fn eat_variable_use_identifier(
        &mut self,
        predefined_token: Option<&Token>,
    ) -> Result<&Entry, String> {
        let current_token = match predefined_token {
            Some(token) => token,
            None => self.tokenizer.current_token.as_ref().unwrap(),
        };
        match current_token {
            Token::Identifier { literal } => {
                let symbol = self.symbol_table.get_entry(literal);
                match symbol {
                    Some(entry) => {
                        if predefined_token.is_none() {
                            self.tokenizer.advance();
                        }
                        Ok(entry)
                    }
                    None => Err(format!(
                        "Use: Couldn't find \"{}\" in symbol table",
                        literal,
                    )),
                }
            }
            _ => Err(format!("Token is not an Identifier: {:?}", current_token)),
        }
    }

    fn eat_identifier(&mut self, predefined_token: Option<&Token>) -> Result<Token, String> {
        let current_token = match predefined_token {
            Some(token) => token,
            None => self.tokenizer.current_token.as_ref().unwrap(),
        }
        .clone();
        if let Token::Identifier { .. } = current_token {
            if predefined_token.is_none() {
                self.tokenizer.advance();
            }
            return Ok(current_token);
        } else {
            return Err(format!("Token is not an Identifier: {:?}", current_token));
        }
    }

    pub fn compile_class(&mut self) -> Result<(), String> {
        /* Compiles a complete class.
         * Since this is the entry point to the compilation engine, first we
         * must advance to the first token.
         */
        self.tokenizer.advance();

        self.eat_keyword_or_symbol(
            vec![Token::Keyword {
                keyword: Keyword::Class,
            }],
            None,
        )?;
        let class_name = self.eat_identifier(None)?;
        if let Token::Identifier { literal } = class_name {
            self.class_name = literal;
        }
        self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::LCurly,
            }],
            None,
        )?;
        self.compile_all_class_var_dec()?;
        self.compile_all_subroutine_dec()?;
        self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::RCurly,
            }],
            None,
        )?;
        Ok(())
    }

    fn eat_type(&mut self) -> Result<Token, String> {
        let mut res = self.eat_keyword_or_symbol(
            vec![
                Token::Keyword {
                    keyword: Keyword::Int,
                },
                Token::Keyword {
                    keyword: Keyword::Char,
                },
                Token::Keyword {
                    keyword: Keyword::Boolean,
                },
            ],
            None,
        );
        if res.is_err() {
            // Couldn't eat int, char, or boolean. Attempt to eat className
            res = self.eat_identifier(None);
            if res.is_err() {
                return Err(format!(
                    "current_token: {:?}  expected_token: type",
                    self.tokenizer.current_token.as_ref()
                ));
            }
        }
        res
    }

    fn compile_class_var_dec(&mut self) -> Result<(), String> {
        // Compiles a static variable declaration, or a field declaration.
        let static_or_field = self.eat_keyword_or_symbol(
            vec![
                Token::Keyword {
                    keyword: Keyword::Static,
                },
                Token::Keyword {
                    keyword: Keyword::Field,
                },
            ],
            None,
        )?;
        let kind = keyword_to_kind(&static_or_field)?;
        let type_ = self.eat_type()?;
        let type_ = type_to_string(&type_)?;
        self.eat_variable_definition_identifier(kind.clone(), type_.clone())?;
        while let Ok(_) = self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::Comma,
            }],
            None,
        ) {
            self.eat_variable_definition_identifier(kind.clone(), type_.clone())?;
        }
        self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::Semicolon,
            }],
            None,
        )?;
        Ok(())
    }

    fn compile_all_class_var_dec(&mut self) -> Result<(), String> {
        // Compiles all class var declarations in a loop.
        while match self.tokenizer.current_token.as_ref().unwrap() {
            Token::Keyword {
                keyword: Keyword::Static,
            } => true,
            Token::Keyword {
                keyword: Keyword::Field,
            } => true,
            _ => false,
        } {
            self.compile_class_var_dec()?;
        }
        Ok(())
    }

    fn compile_subroutine_dec(&mut self) -> Result<(), String> {
        // Compiles a complete method, function, or constructor.
        self.symbol_table.start_subroutine();
        self.if_statement_idx = 0;
        self.while_statement_idx = 0;

        let subroutine_type = self.eat_keyword_or_symbol(
            vec![
                Token::Keyword {
                    keyword: Keyword::Constructor,
                },
                Token::Keyword {
                    keyword: Keyword::Function,
                },
                Token::Keyword {
                    keyword: Keyword::Method,
                },
            ],
            None,
        )?;
        let void_function = match self.eat_type() {
            Ok(_) => false,
            Err(_) => {
                self.eat_keyword_or_symbol(
                    vec![Token::Keyword {
                        keyword: Keyword::Void,
                    }],
                    None,
                )?;
                true
            }
        };
        let subroutine_name = self.eat_identifier(None)?;
        self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::LParen,
            }],
            None,
        )?;
        let is_method = match subroutine_type {
            Token::Keyword { ref keyword } => match keyword {
                Keyword::Method => true,
                _ => false,
            },
            _ => {
                return Err(format!(
                    "Subroutine type is not Keyword: {:?}",
                    subroutine_type
                ))
            }
        };
        self.compile_parameter_list(is_method)?;
        self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::RParen,
            }],
            None,
        )?;
        self.compile_subroutine_body(subroutine_type, subroutine_name, void_function)?;
        Ok(())
    }

    fn compile_all_subroutine_dec(&mut self) -> Result<(), String> {
        // Compiles all subroutine declarations in a loop.
        while match self.tokenizer.current_token.as_ref().unwrap() {
            Token::Keyword {
                keyword: Keyword::Constructor,
            } => true,
            Token::Keyword {
                keyword: Keyword::Function,
            } => true,
            Token::Keyword {
                keyword: Keyword::Method,
            } => true,
            _ => false,
        } {
            self.compile_subroutine_dec()?;
        }
        Ok(())
    }

    fn compile_parameter_list(&mut self, is_method: bool) -> Result<(), String> {
        // Compiles a (possibly empty) parameter list. Does not handle the enclosing "()".
        if is_method {
            self.symbol_table
                .define(String::from("self"), self.class_name.clone(), Kind::Arg);
        }

        let res = self.eat_type();
        if let Ok(token) = res {
            let type_ = type_to_string(&token)?;
            self.eat_variable_definition_identifier(Kind::Arg, type_)?;
            while let Ok(_) = self.eat_keyword_or_symbol(
                vec![Token::Symbol {
                    symbol: Symbol::Comma,
                }],
                None,
            ) {
                let type_ = self.eat_type()?;
                let type_ = type_to_string(&type_)?;
                self.eat_variable_definition_identifier(Kind::Arg, type_)?;
            }
        }
        Ok(())
    }

    fn compile_subroutine_body(
        &mut self,
        subroutine_type: Token,
        subroutine_name: Token,
        void_function: bool,
    ) -> Result<(), String> {
        // Compiles a subroutine's body.
        self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::LCurly,
            }],
            None,
        )?;

        self.compile_all_var_dec()?;
        if let Token::Identifier { literal } = subroutine_name {
            let function_name = format!("{}.{}", self.class_name, literal);
            self.vm_writer
                .write_function(function_name, self.symbol_table.subroutine_index_var);
        }

        let ctor_or_method = match subroutine_type {
            Token::Keyword { keyword } => match keyword {
                Keyword::Constructor => {
                    self.vm_writer
                        .write_push(MemorySegment::Constant, self.num_fields);
                    self.vm_writer.write_call(ALLOC_FN.to_owned(), 1);
                    self.vm_writer.write_pop(MemorySegment::Pointer, 0);
                    true
                }
                Keyword::Method => {
                    self.vm_writer.write_push(MemorySegment::Argument, 0);
                    self.vm_writer.write_pop(MemorySegment::Pointer, 0);
                    true
                }
                _ => false,
            },
            _ => {
                return Err(format!(
                    "Subroutine type is not Keyword: {:?}",
                    subroutine_type
                ))
            }
        };

        self.compile_statements(ctor_or_method, void_function)?;
        self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::RCurly,
            }],
            None,
        )?;
        Ok(())
    }

    fn compile_var_dec(&mut self) -> Result<(), String> {
        // Compiles a `var` declaration.
        self.eat_keyword_or_symbol(
            vec![Token::Keyword {
                keyword: Keyword::Var,
            }],
            None,
        )?;
        let type_ = self.eat_type()?;
        let type_ = type_to_string(&type_)?;
        self.eat_variable_definition_identifier(Kind::Var, type_.clone())?;
        while let Ok(_) = self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::Comma,
            }],
            None,
        ) {
            self.eat_variable_definition_identifier(Kind::Var, type_.clone())?;
        }
        self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::Semicolon,
            }],
            None,
        )?;
        Ok(())
    }

    fn compile_all_var_dec(&mut self) -> Result<(), String> {
        // Compiles all var declarations in a loop.
        while self.tokenizer.current_token.as_ref().unwrap()
            == &(Token::Keyword {
                keyword: Keyword::Var,
            })
        {
            self.compile_var_dec()?;
        }
        Ok(())
    }

    fn compile_statements(
        &mut self,
        ctor_or_method: bool,
        void_function: bool,
    ) -> Result<(), String> {
        // Compiles a sequence of statments. Does not handle the enclosing "{}".
        let mut statements_left = true;
        while statements_left {
            let current_token = self.tokenizer.current_token.as_ref().unwrap();
            match current_token {
                Token::Keyword { keyword } => match keyword {
                    Keyword::Let => self.compile_let()?,
                    Keyword::If => self.compile_if(ctor_or_method, void_function)?,
                    Keyword::While => self.compile_while(ctor_or_method, void_function)?,
                    Keyword::Do => self.compile_do(ctor_or_method)?,
                    Keyword::Return => self.compile_return(void_function)?,
                    _ => statements_left = false,
                },
                _ => statements_left = false,
            }
        }
        Ok(())
    }

    fn compile_let(&mut self) -> Result<(), String> {
        // Compiles a let statement.
        self.eat_keyword_or_symbol(
            vec![Token::Keyword {
                keyword: Keyword::Let,
            }],
            None,
        )?;
        let symbol = self.eat_variable_use_identifier(None)?;
        let segment = kind_to_segment(&symbol.kind);
        let index = symbol.index;
        let res = self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::Equals,
            }],
            None,
        );
        if res.is_err() {
            // The let statement is assigning to an array entry.
            self.eat_keyword_or_symbol(
                vec![Token::Symbol {
                    symbol: Symbol::LBracket,
                }],
                None,
            )?;

            self.compile_expression()?;
            self.vm_writer.write_push(segment, index);
            self.vm_writer.write_arithmetic(MathInstr::Add);

            self.eat_keyword_or_symbol(
                vec![Token::Symbol {
                    symbol: Symbol::RBracket,
                }],
                None,
            )?;
            self.eat_keyword_or_symbol(
                vec![Token::Symbol {
                    symbol: Symbol::Equals,
                }],
                None,
            )?;

            self.compile_expression()?;
            self.vm_writer.write_pop(MemorySegment::Temp, 0);
            self.vm_writer.write_pop(MemorySegment::Pointer, 1);
            self.vm_writer.write_push(MemorySegment::Temp, 0);
            self.vm_writer.write_pop(MemorySegment::That, 0);
        } else {
            // The let statement is assigning to an variable.
            self.compile_expression()?;
            self.vm_writer.write_pop(segment, index);
        }

        self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::Semicolon,
            }],
            None,
        )?;
        Ok(())
    }

    fn compile_if(&mut self, ctor_or_method: bool, void_function: bool) -> Result<(), String> {
        // Compiles an if statement, possibly with a trailing `else` clause.
        let label_if_true = format!("IF_TRUE{}", self.if_statement_idx);
        let label_if_false = format!("IF_FALSE{}", self.if_statement_idx);
        let label_if_end = format!("IF_END{}", self.if_statement_idx);
        self.if_statement_idx += 1;

        self.eat_keyword_or_symbol(
            vec![Token::Keyword {
                keyword: Keyword::If,
            }],
            None,
        )?;
        self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::LParen,
            }],
            None,
        )?;

        self.compile_expression()?;
        self.vm_writer.write_if_goto(label_if_true.clone());
        self.vm_writer.write_goto(label_if_false.clone());
        self.vm_writer.write_label(label_if_true);

        self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::RParen,
            }],
            None,
        )?;
        self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::LCurly,
            }],
            None,
        )?;

        self.compile_statements(ctor_or_method, void_function)?;

        self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::RCurly,
            }],
            None,
        )?;
        let res = self.eat_keyword_or_symbol(
            vec![Token::Keyword {
                keyword: Keyword::Else,
            }],
            None,
        );
        if res.is_ok() {
            // Entering else statement
            self.eat_keyword_or_symbol(
                vec![Token::Symbol {
                    symbol: Symbol::LCurly,
                }],
                None,
            )?;

            self.vm_writer.write_goto(label_if_end.clone());
            self.vm_writer.write_label(label_if_false);
            self.compile_statements(ctor_or_method, void_function)?;
            self.vm_writer.write_label(label_if_end);

            self.eat_keyword_or_symbol(
                vec![Token::Symbol {
                    symbol: Symbol::RCurly,
                }],
                None,
            )?;
        } else {
            // No else statement exists
            self.vm_writer.write_label(label_if_false);
        }
        Ok(())
    }

    fn compile_while(&mut self, ctor_or_method: bool, void_function: bool) -> Result<(), String> {
        // Compiles a while statement.
        let label_expr = format!("WHILE_EXP{}", self.while_statement_idx);
        let label_end = format!("WHILE_END{}", self.while_statement_idx);
        self.while_statement_idx += 1;

        self.eat_keyword_or_symbol(
            vec![Token::Keyword {
                keyword: Keyword::While,
            }],
            None,
        )?;
        self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::LParen,
            }],
            None,
        )?;

        self.vm_writer.write_label(label_expr.clone());
        self.compile_expression()?;
        self.vm_writer.write_arithmetic(MathInstr::Not);
        self.vm_writer.write_if_goto(label_end.clone());

        self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::RParen,
            }],
            None,
        )?;
        self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::LCurly,
            }],
            None,
        )?;

        self.compile_statements(ctor_or_method, void_function)?;
        self.vm_writer.write_goto(label_expr);
        self.vm_writer.write_label(label_end);

        self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::RCurly,
            }],
            None,
        )?;
        Ok(())
    }

    fn call_subroutine(&mut self, class_name: &str, func_name: &str) -> Result<(), String> {
        let symbol = self.symbol_table.get_entry(class_name);
        match symbol {
            Some(entry) => {
                // We are calling a method
                self.vm_writer
                    .write_push(kind_to_segment(&entry.kind), entry.index);
                let type_ = entry.type_.clone();
                let num_args = 1 + self.compile_expression_list()?;
                self.vm_writer
                    .write_call(format!("{}.{}", type_, func_name), num_args);
            }
            None => {
                // We are not calling a method
                let num_args = self.compile_expression_list()?;
                self.vm_writer
                    .write_call(format!("{}.{}", class_name, func_name), num_args);
            }
        }
        Ok(())
    }

    fn compile_do(&mut self, ctor_or_method: bool) -> Result<(), String> {
        // Compiles a do statement.
        self.eat_keyword_or_symbol(
            vec![Token::Keyword {
                keyword: Keyword::Do,
            }],
            None,
        )?;
        let initial_identifier = self.eat_identifier(None)?;
        let left_paren = self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::LParen,
            }],
            None,
        );
        match left_paren {
            Ok(token) => {
                self.eat_identifier(Some(&initial_identifier))?;
                self.eat_keyword_or_symbol(
                    vec![Token::Symbol {
                        symbol: Symbol::LParen,
                    }],
                    Some(&token),
                )?;
                match initial_identifier {
                    Token::Identifier { literal } => {
                        if ctor_or_method {
                            self.vm_writer.write_push(MemorySegment::Pointer, 0);
                            let num_args = 1 + self.compile_expression_list()?;
                            self.vm_writer
                                .write_call(format!("{}.{}", self.class_name, literal), num_args)
                        } else {
                            return Err(format!("Self method called from function"));
                        }
                    }
                    _ => {
                        return Err(format!(
                            "Subroutine name is not identifier: {:?}",
                            initial_identifier
                        ))
                    }
                }
            }
            Err(_) => {
                self.eat_identifier(Some(&initial_identifier))?;
                self.eat_keyword_or_symbol(
                    vec![Token::Symbol {
                        symbol: Symbol::Period,
                    }],
                    None,
                )?;
                let second_identifier = self.eat_identifier(None)?;
                self.eat_keyword_or_symbol(
                    vec![Token::Symbol {
                        symbol: Symbol::LParen,
                    }],
                    None,
                )?;
                match initial_identifier {
                    Token::Identifier {
                        literal: class_name,
                    } => match second_identifier {
                        Token::Identifier { literal: func_name } => {
                            self.call_subroutine(&class_name, &func_name)?;
                        }
                        _ => {
                            return Err(format!(
                                "Subroutine name is not identifier: {:?}",
                                second_identifier
                            ))
                        }
                    },
                    _ => {
                        return Err(format!(
                            "Class name is not identifier: {:?}",
                            initial_identifier
                        ))
                    }
                }
            }
        };

        // Since we are calling a void function, we need to pop the return
        // value off the stack
        self.vm_writer.write_pop(MemorySegment::Temp, 0);

        self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::RParen,
            }],
            None,
        )?;
        self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::Semicolon,
            }],
            None,
        )?;
        Ok(())
    }

    fn compile_return(&mut self, void_function: bool) -> Result<(), String> {
        // Compiles a return statement.
        self.eat_keyword_or_symbol(
            vec![Token::Keyword {
                keyword: Keyword::Return,
            }],
            None,
        )?;
        let res = self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::Semicolon,
            }],
            None,
        );
        if res.is_err() {
            self.compile_expression()?;
            self.eat_keyword_or_symbol(
                vec![Token::Symbol {
                    symbol: Symbol::Semicolon,
                }],
                None,
            )?;
        }

        // If the function's return type is void, we need to push a default
        // return value to the stack according to the function call-and-return
        // contract.
        if void_function {
            self.vm_writer.write_push(MemorySegment::Constant, 0);
        }
        self.vm_writer.write_return();
        Ok(())
    }

    fn compile_expression(&mut self) -> Result<(), String> {
        // Compiles an expression.
        self.compile_term()?;
        while let Ok(op) = self.eat_keyword_or_symbol(
            vec![
                Token::Symbol {
                    symbol: Symbol::Plus,
                },
                Token::Symbol {
                    symbol: Symbol::Minus,
                },
                Token::Symbol {
                    symbol: Symbol::Asterisk,
                },
                Token::Symbol {
                    symbol: Symbol::Slash,
                },
                Token::Symbol {
                    symbol: Symbol::Ampersand,
                },
                Token::Symbol {
                    symbol: Symbol::Pipe,
                },
                Token::Symbol {
                    symbol: Symbol::LessThan,
                },
                Token::Symbol {
                    symbol: Symbol::GreaterThan,
                },
                Token::Symbol {
                    symbol: Symbol::Equals,
                },
            ],
            None,
        ) {
            self.compile_term()?;
            match op {
                Token::Symbol { symbol } => match symbol {
                    Symbol::Plus => self.vm_writer.write_arithmetic(MathInstr::Add),
                    Symbol::Minus => self.vm_writer.write_arithmetic(MathInstr::Sub),
                    Symbol::Asterisk => self.vm_writer.write_call(MULTIPLY_FN.to_owned(), 2),
                    Symbol::Slash => self.vm_writer.write_call(DIVIDE_FN.to_owned(), 2),
                    Symbol::Ampersand => self.vm_writer.write_arithmetic(MathInstr::And),
                    Symbol::Pipe => self.vm_writer.write_arithmetic(MathInstr::Or),
                    Symbol::LessThan => self.vm_writer.write_arithmetic(MathInstr::Lt),
                    Symbol::GreaterThan => self.vm_writer.write_arithmetic(MathInstr::Gt),
                    Symbol::Equals => self.vm_writer.write_arithmetic(MathInstr::Eq),
                    _ => {
                        return Err(format!(
                            "Op Symbol in expression is not implemented: {:?}",
                            symbol
                        ))
                    }
                },
                _ => return Err(format!("Op in expression is not a Symbol: {:?}", op)),
            }
        }
        Ok(())
    }

    fn compile_term(&mut self) -> Result<(), String> {
        /*
         * Compiles a `term`. If the current token is an `identifier`, the
         * routine must distinguish between a `variable`, an `array entry`, or
         * a `subroutine call`. A single look-ahead token, which may be one of
         * "[", "(", or ".", suffices to distinguish between the
         * possibilities. Any other token is not part of this term and should
         * not be advanced over.
         */
        // Always attempt to eat unaryOp first as they can precede any term.
        let res = self.eat_keyword_or_symbol(
            vec![
                Token::Symbol {
                    symbol: Symbol::Minus,
                },
                Token::Symbol {
                    symbol: Symbol::Tilde,
                },
            ],
            None,
        );
        if res.is_ok() {
            self.compile_term()?;
            match res {
                Ok(Token::Symbol { symbol }) => match symbol {
                    Symbol::Minus => self.vm_writer.write_arithmetic(MathInstr::Neg),
                    Symbol::Tilde => self.vm_writer.write_arithmetic(MathInstr::Not),
                    _ => return Err(format!("UnaryOp is not neg or not: {:?}", symbol)),
                },
                _ => return Err(format!("UnaryOp is not Symbol: {:?}", res)),
            }
            return Ok(());
        }

        // First, attempt to eat another expression within parentheses.
        let res = self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::LParen,
            }],
            None,
        );
        if res.is_ok() {
            self.compile_expression()?;
            self.eat_keyword_or_symbol(
                vec![Token::Symbol {
                    symbol: Symbol::RParen,
                }],
                None,
            )?;
            return Ok(());
        }

        // Second, attempt to eat either integerConstant or stringConstant.
        let res = self.eat_integer();
        if let Ok(Token::IntegerConstant { value }) = res {
            self.vm_writer.write_push(MemorySegment::Constant, value);
            return Ok(());
        }
        let res = self.eat_string();
        if let Ok(Token::StringConstant { literal }) = res {
            let len = literal.len();
            self.vm_writer.write_push(MemorySegment::Constant, len);
            self.vm_writer.write_call(STRING_NEW_FN.to_owned(), 1);
            for ch in literal.chars() {
                self.vm_writer
                    .write_push(MemorySegment::Constant, ch as usize);
                self.vm_writer.write_call(STRING_APPEND_FN.to_owned(), 2);
            }
            return Ok(());
        }

        // Third, attempt to eat keywordConstant.
        let res = self.eat_keyword_or_symbol(
            vec![
                Token::Keyword {
                    keyword: Keyword::True,
                },
                Token::Keyword {
                    keyword: Keyword::False,
                },
                Token::Keyword {
                    keyword: Keyword::Null,
                },
                Token::Keyword {
                    keyword: Keyword::This,
                },
            ],
            None,
        );
        if res.is_ok() {
            match res {
                Ok(Token::Keyword { keyword }) => match keyword {
                    Keyword::True => {
                        self.vm_writer.write_push(MemorySegment::Constant, 0);
                        self.vm_writer.write_arithmetic(MathInstr::Not);
                    }
                    Keyword::False => self.vm_writer.write_push(MemorySegment::Constant, 0),
                    Keyword::Null => self.vm_writer.write_push(MemorySegment::Constant, 0),
                    Keyword::This => self.vm_writer.write_push(MemorySegment::Pointer, 0),
                    _ => return Err(format!("Keyword constant not implemented: {:?}", keyword)),
                },
                _ => return Err(format!("Keyword constant is not Keyword: {:?}", res)),
            }
            return Ok(());
        }

        // Once all the previous options have been exhausted, the next token
        // must be an identifier.
        let initial_identifier = self.eat_identifier(None)?;

        // The identifier can be a variable, array entry, or subroutine call
        // based on the next token.
        // First, check if it's an array entry.
        let left_bracket = self.eat_keyword_or_symbol(
            vec![Token::Symbol {
                symbol: Symbol::LBracket,
            }],
            None,
        );
        if left_bracket.is_ok() {
            let symbol = self.eat_variable_use_identifier(Some(&initial_identifier))?;
            let segment = kind_to_segment(&symbol.kind);
            let index = symbol.index;
            self.eat_keyword_or_symbol(
                vec![Token::Symbol {
                    symbol: Symbol::LBracket,
                }],
                Some(&left_bracket.unwrap()),
            )?;

            self.compile_expression()?;
            self.vm_writer.write_push(segment, index);
            self.vm_writer.write_arithmetic(MathInstr::Add);
            self.vm_writer.write_pop(MemorySegment::Pointer, 1);
            self.vm_writer.write_push(MemorySegment::That, 0);

            self.eat_keyword_or_symbol(
                vec![Token::Symbol {
                    symbol: Symbol::RBracket,
                }],
                None,
            )?;
            return Ok(());
        }

        // Second, check if it's a subroutine call or a variable.
        let next_token = self.eat_keyword_or_symbol(
            vec![
                Token::Symbol {
                    symbol: Symbol::LParen,
                },
                Token::Symbol {
                    symbol: Symbol::Period,
                },
            ],
            None,
        );
        match next_token {
            Ok(token) => match token {
                // Term is a subroutine call
                Token::Symbol { ref symbol } => match symbol {
                    Symbol::LParen => {
                        self.eat_identifier(Some(&initial_identifier))?;
                        self.eat_keyword_or_symbol(
                            vec![Token::Symbol {
                                symbol: Symbol::LParen,
                            }],
                            Some(&token),
                        )?;

                        let num_args = self.compile_expression_list()?;
                        let subroutine_name = match initial_identifier {
                            Token::Identifier { literal } => literal,
                            _ => {
                                return Err(format!(
                                    "Subroutine name is not identifier: {:?}",
                                    initial_identifier
                                ))
                            }
                        };
                        self.vm_writer.write_call(subroutine_name, num_args);

                        self.eat_keyword_or_symbol(
                            vec![Token::Symbol {
                                symbol: Symbol::RParen,
                            }],
                            None,
                        )?;
                        return Ok(());
                    }
                    Symbol::Period => {
                        self.eat_identifier(Some(&initial_identifier))?;
                        self.eat_keyword_or_symbol(
                            vec![Token::Symbol {
                                symbol: Symbol::Period,
                            }],
                            Some(&token),
                        )?;
                        let second_identifier = self.eat_identifier(None)?;
                        self.eat_keyword_or_symbol(
                            vec![Token::Symbol {
                                symbol: Symbol::LParen,
                            }],
                            None,
                        )?;

                        match initial_identifier {
                            Token::Identifier {
                                literal: class_name,
                            } => match second_identifier {
                                Token::Identifier { literal: func_name } => {
                                    self.call_subroutine(&class_name, &func_name)?;
                                }
                                _ => {
                                    return Err(format!(
                                        "Subroutine name is not identifier: {:?}",
                                        second_identifier
                                    ))
                                }
                            },
                            _ => {
                                return Err(format!(
                                    "Class name is not identifier: {:?}",
                                    initial_identifier
                                ))
                            }
                        };

                        self.eat_keyword_or_symbol(
                            vec![Token::Symbol {
                                symbol: Symbol::RParen,
                            }],
                            None,
                        )?;
                        return Ok(());
                    }
                    _ => return Err(format!("Eating a LParen or Period returned: {:?}", symbol)),
                },
                _ => return Err(format!("Eating a Symbol returned: {:?}", token)),
            },
            Err(_) => {
                // If it's not an array index or a subroutine call, it is a variable.
                let symbol = self.eat_variable_use_identifier(Some(&initial_identifier))?;
                let segment = kind_to_segment(&symbol.kind);
                let index = symbol.index;
                self.vm_writer.write_push(segment, index);
            }
        }
        Ok(())
    }

    fn current_token_begins_term(&self) -> bool {
        let current_token = self.tokenizer.current_token.as_ref().unwrap();
        match current_token {
            Token::IntegerConstant { .. } => true,
            Token::StringConstant { .. } => true,
            Token::Keyword {
                keyword: Keyword::True,
            } => true,
            Token::Keyword {
                keyword: Keyword::False,
            } => true,
            Token::Keyword {
                keyword: Keyword::Null,
            } => true,
            Token::Keyword {
                keyword: Keyword::This,
            } => true,
            Token::Identifier { .. } => true,
            Token::Symbol {
                symbol: Symbol::LParen,
            } => true,
            Token::Symbol {
                symbol: Symbol::Minus,
            } => true,
            Token::Symbol {
                symbol: Symbol::Tilde,
            } => true,
            _ => false,
        }
    }

    fn compile_expression_list(&mut self) -> Result<usize, String> {
        // Compiles a (possibly empty) comma-separated list of expressions.
        // Returns the number of comma-separated expressions.
        let mut num_expressions: usize = 0;
        if self.current_token_begins_term() {
            self.compile_expression()?;
            num_expressions += 1;
            while let Ok(_) = self.eat_keyword_or_symbol(
                vec![Token::Symbol {
                    symbol: Symbol::Comma,
                }],
                None,
            ) {
                self.compile_expression()?;
                num_expressions += 1;
            }
        }
        Ok(num_expressions)
    }
}
