use crate::syntax_analyzer::tokenizer::{Keyword, Symbol, Token, Tokenizer};

pub struct CompilationEngine {
    tokenizer: Tokenizer,
    pub result: Vec<String>,
}

impl CompilationEngine {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self {
            tokenizer: tokenizer,
            result: vec![],
        }
    }

    fn add_xml_event<Event: Into<String>>(&mut self, event: Event) {
        let event = event.into();
        self.result.push(event);
    }

    fn add_xml_events(&mut self, events: Vec<String>) {
        self.result.extend(events);
    }

    fn eat_keyword_or_symbol(&mut self, expected_tokens: Vec<Token>) -> Result<(), String> {
        // Checks if the current token is one of the expected tokens (keywords or symbols).
        let current_token = self.tokenizer.current_token.as_ref().unwrap();
        if expected_tokens.contains(current_token) {
            self.add_xml_events(current_token.to_xml_events());
            self.tokenizer.advance();
            return Ok(());
        } else {
            return Err(format!(
                "current_token: {:?}  expected_token: {:?}",
                current_token, expected_tokens
            ));
        }
    }

    fn eat_integer(&mut self) -> Result<(), String> {
        let current_token = self.tokenizer.current_token.as_ref().unwrap();
        if let Token::IntegerConstant { .. } = current_token {
            self.add_xml_events(current_token.to_xml_events());
            self.tokenizer.advance();
            return Ok(());
        } else {
            return Err(format!(
                "Token is not an IntegerConstant: {:?}",
                current_token
            ));
        }
    }

    fn eat_string(&mut self) -> Result<(), String> {
        let current_token = self.tokenizer.current_token.as_ref().unwrap();
        if let Token::StringConstant { .. } = current_token {
            self.add_xml_events(current_token.to_xml_events());
            self.tokenizer.advance();
            return Ok(());
        } else {
            return Err(format!(
                "Token is not an StringConstant: {:?}",
                current_token
            ));
        }
    }

    fn eat_identifier(&mut self) -> Result<(), String> {
        let current_token = self.tokenizer.current_token.as_ref().unwrap();
        if let Token::Identifier { .. } = current_token {
            self.add_xml_events(current_token.to_xml_events());
            self.tokenizer.advance();
            return Ok(());
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
        self.add_xml_event("+class");

        self.eat_keyword_or_symbol(vec![Token::Keyword {
            keyword: Keyword::Class,
        }])?;
        self.eat_identifier()?;
        self.eat_keyword_or_symbol(vec![Token::Symbol {
            symbol: Symbol::LCurly,
        }])?;
        self.compile_all_class_var_dec()?;
        self.compile_subroutine_dec()?;

        self.add_xml_event("-class");
        Ok(())
    }

    fn eat_type(&mut self) -> Result<(), String> {
        let res = self.eat_keyword_or_symbol(vec![
            Token::Keyword {
                keyword: Keyword::Int,
            },
            Token::Keyword {
                keyword: Keyword::Char,
            },
            Token::Keyword {
                keyword: Keyword::Boolean,
            },
        ]);
        if res.is_err() {
            // Couldn't eat int, char, or boolean. Attempt to eat className
            let res = self.eat_identifier();
            if res.is_err() {
                return Err(format!(
                    "current_token: {:?}  expected_token: type",
                    self.tokenizer.current_token.as_ref()
                ));
            }
        }
        Ok(())
    }

    fn compile_class_var_dec(&mut self) -> Result<(), String> {
        // Compiles a static variable declaration, or a field declaration.
        self.add_xml_event("+classVarDec");

        self.eat_keyword_or_symbol(vec![
            Token::Keyword {
                keyword: Keyword::Static,
            },
            Token::Keyword {
                keyword: Keyword::Field,
            },
        ])?;
        self.eat_type()?;
        self.eat_identifier()?;
        while let Ok(()) = self.eat_keyword_or_symbol(vec![Token::Symbol {
            symbol: Symbol::Comma,
        }]) {
            self.eat_identifier()?;
        }
        self.eat_keyword_or_symbol(vec![Token::Symbol {
            symbol: Symbol::Semicolon,
        }])?;

        self.add_xml_event("-classVarDec");
        Ok(())
    }

    fn compile_all_class_var_dec(&mut self) -> Result<(), String> {
        // Compiles all class var declarations in a loop.
        let class_var_decs = vec![
            Token::Keyword {
                keyword: Keyword::Static,
            },
            Token::Keyword {
                keyword: Keyword::Field,
            },
        ];
        while class_var_decs.contains(self.tokenizer.current_token.as_ref().unwrap()) {
            self.compile_class_var_dec()?;
        }
        Ok(())
    }

    fn compile_subroutine_dec(&mut self) -> Result<(), String> {
        // Compiles a complete method, function, or constructor.
        self.add_xml_event("+subroutineDec");

        self.eat_keyword_or_symbol(vec![
            Token::Keyword {
                keyword: Keyword::Constructor,
            },
            Token::Keyword {
                keyword: Keyword::Function,
            },
            Token::Keyword {
                keyword: Keyword::Method,
            },
        ])?;
        let res = self.eat_type();
        if res.is_err() {
            self.eat_keyword_or_symbol(vec![Token::Keyword {
                keyword: Keyword::Void,
            }])?;
        }
        self.eat_identifier()?;
        self.eat_keyword_or_symbol(vec![Token::Symbol {
            symbol: Symbol::LParen,
        }])?;
        self.compile_parameter_list()?;
        self.eat_keyword_or_symbol(vec![Token::Symbol {
            symbol: Symbol::RParen,
        }])?;
        self.compile_subroutine_body()?;

        self.add_xml_event("-subroutineDec");
        Ok(())
    }

    fn compile_parameter_list(&mut self) -> Result<(), String> {
        // Compiles a (possibly empty) parameter list. Does not handle the enclosing "()".
        self.add_xml_event("+parameterList");

        let res = self.eat_type();
        if res.is_ok() {
            self.eat_identifier()?;
            while let Ok(()) = self.eat_keyword_or_symbol(vec![Token::Symbol {
                symbol: Symbol::Comma,
            }]) {
                self.eat_type()?;
                self.eat_identifier()?;
            }
        }

        self.add_xml_event("-parameterList");
        Ok(())
    }

    fn compile_subroutine_body(&mut self) -> Result<(), String> {
        // Compiles a subroutine's body.
        self.add_xml_event("+subroutineBody");

        self.eat_keyword_or_symbol(vec![Token::Symbol {
            symbol: Symbol::LCurly,
        }])?;
        self.compile_all_var_dec()?;
        self.compile_statements()?;
        // self.eat_keyword_or_symbol(vec![Token::Symbol {
        //     symbol: Symbol::RCurly,
        // }])?;

        self.add_xml_event("-subroutineBody");
        Ok(())
    }

    fn compile_var_dec(&mut self) -> Result<(), String> {
        // Compiles a `var` declaration.
        self.add_xml_event("+varDec");

        self.eat_keyword_or_symbol(vec![Token::Keyword {
            keyword: Keyword::Var,
        }])?;
        self.eat_type()?;
        self.eat_identifier()?;
        while let Ok(()) = self.eat_keyword_or_symbol(vec![Token::Symbol {
            symbol: Symbol::Comma,
        }]) {
            self.eat_identifier()?;
        }
        self.eat_keyword_or_symbol(vec![Token::Symbol {
            symbol: Symbol::Semicolon,
        }])?;

        self.add_xml_event("-varDec");
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

    fn compile_statements(&mut self) -> Result<(), String> {
        // Compiles a sequence of statments. Does not handle the enclosing "{}".
        self.add_xml_event("+statements");

        let mut statements_left = true;
        while statements_left {
            let current_token = self.tokenizer.current_token.as_ref().unwrap();
            match current_token {
                Token::Keyword { keyword } => match keyword {
                    Keyword::Let => self.compile_let()?,
                    // Keyword::If => self.compile_if()?,
                    // Keyword::While => self.compile_while()?,
                    // Keyword::Do => self.compile_do()?,
                    // Keyword::Return => self.compile_return()?,
                    _ => statements_left = false,
                },
                _ => statements_left = false,
            }
        }

        self.add_xml_event("-statements");
        Ok(())
    }

    fn compile_let(&mut self) -> Result<(), String> {
        // Compiles a let statement.
        self.add_xml_event("+letStatement");

        self.eat_keyword_or_symbol(vec![Token::Keyword {
            keyword: Keyword::Let,
        }])?;
        self.eat_identifier()?;
        self.eat_keyword_or_symbol(vec![Token::Symbol {
            symbol: Symbol::Equals,
        }])?;
        self.compile_expression()?;
        self.eat_keyword_or_symbol(vec![Token::Symbol {
            symbol: Symbol::Semicolon,
        }])?;

        self.add_xml_event("-letStatement");
        Ok(())
    }

    fn compile_if(&mut self) -> Result<(), String> {
        // Compiles an if statement, possibly with a trailing `else` clause.
        self.add_xml_event("+ifStatement");

        self.eat_keyword_or_symbol(vec![Token::Keyword {
            keyword: Keyword::If,
        }])?;

        self.add_xml_event("-ifStatement");
        Ok(())
    }

    fn compile_while(&mut self) -> Result<(), String> {
        // Compiles a while statement.
        self.add_xml_event("+whileStatement");

        self.eat_keyword_or_symbol(vec![Token::Keyword {
            keyword: Keyword::While,
        }])?;

        self.add_xml_event("-whileStatement");
        Ok(())
    }

    fn compile_do(&mut self) -> Result<(), String> {
        // Compiles a do statement.
        self.add_xml_event("+doStatement");

        self.eat_keyword_or_symbol(vec![Token::Keyword {
            keyword: Keyword::Do,
        }])?;

        self.add_xml_event("-doStatement");
        Ok(())
    }

    fn compile_return(&mut self) -> Result<(), String> {
        // Compiles a return statement.
        self.add_xml_event("+returnStatement");

        self.eat_keyword_or_symbol(vec![Token::Keyword {
            keyword: Keyword::Return,
        }])?;

        self.add_xml_event("-returnStatement");
        Ok(())
    }

    fn compile_expression(&mut self) -> Result<(), String> {
        // Compiles an expression.
        self.add_xml_event("+expression");

        self.compile_term()?;

        self.add_xml_event("-expression");
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
        self.add_xml_event("+term");

        self.eat_identifier()?;

        self.add_xml_event("-term");
        Ok(())
    }

    fn compile_expression_list(&mut self) -> Result<(), String> {
        // Compiles a (possibly empty) comma-separated list of expressions.
        self.add_xml_event("+expressionList");

        self.add_xml_event("-expressionList");
        Ok(())
    }
}
