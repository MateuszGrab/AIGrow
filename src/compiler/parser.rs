use crate::compiler::lexer::{Token};
use crate::compiler::lexer::NumberLiteral;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken { token: Token, message: String },
    // Dodaj więcej rodzajów błędów według potrzeb
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
    // Dodaj więcej rodzajów priorytetów według potrzeb
}


pub enum Expr {
    Literal(Literal),
    Identifier(String),
    Binary(Box<Expr>, BinaryOperator, Box<Expr>),
    Grouping(Box<Expr>),
    Unary(UnaryOperator, Box<Expr>),
    Conditional(Box<Expr>, Box<Expr>, Box<Expr>),
    // Dodaj więcej wyrażeń według potrzeb
}

pub enum Literal {
    NumberLiteral(NumberLiteral),
    StringLiteral(String),
    BoolLiteral(bool),
    // Dodaj więcej literalów według potrzeb
}

pub enum UnaryOperator {
    Negate,
    Not,
    // Dodaj więcej operatorów według potrzeb
}

pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    // Dodaj więcej operatorów według potrzeb
}

pub enum Stmt {
    PureFunctionDecl {
        name: String,
        params: Vec<(String, Token)>,
        return_type: Token,
    },
    LetDecl {
        name: String,
        value: Expr,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    PureFunctionDeclaration {
        name: Token,
        parameters: Vec<Parameter>,
        return_type: Type,
        body: Vec<Stmt>,
    },
    // Dodaj więcej instrukcji według potrzeb
}
// Sekcja 2
pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

pub struct Parameter {
    name: Token,
    param_type: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    // Add other types as needed
    Int,
    Float,
    Bool,
    String,
    // ...
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => return Err(e),
            }
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Stmt, String> {
        // Depending on the token, parse different types of statements
        if self.match_token(&[Token::PureFunctionKeyword]) {
            self.parse_pure_function_declaration()
        } else if self.match_token(&[Token::Let]) {
            self.parse_let_declaration()
        } else if self.match_token(&[Token::If]) {
            self.parse_if_statement()
        } else if self.match_token(&[Token::While]) {
            self.parse_while_statement()
        } else if self.match_token(&[Token::Return]) {
            self.parse_return_statement()
        } else {
            self.parse_expression()
        }
    }
    
    fn parse_pure_function_declaration(&mut self) -> Result<Stmt, String> {
        // Consuming the 'pf' keyword
        self.consume(Token::PureFunctionKeyword, "Expected 'pf' keyword for pure function declaration.")?;
    
        // Parsing the function name
        let name = self.consume(Token::Identifier("function_name".to_string()), "Expected function name.")?;
        
        // Consuming the '(' token
        self.consume(Token::OpenParen, "Expected '(' after function name.")?;
    
        // Parsing the function parameters
        let mut parameters = Vec::new();
        while let Ok(param) = self.parse_parameter() {
            parameters.push(param);
            // If there's a ',' token, consume it and continue parsing the next parameter
            if
                match self.peek() {
                Token::Comma => matches!(self.peek().token_type, Token::Comma) ,
                _ => panic!(),
                } 
            {
                self.advance();
            }
            else {
                break;
            }
        }
    
        // Consuming the ')' token
        self.consume(Token::CloseParen, "Expected ')' after function parameters.")?;
    
        // Consuming the '->' token
        self.consume(Token::ReturnArrow, "Expected '->' after function parameters.")?;
    
        // Parsing the return type
        let return_type = self.parse_type()?;
    
        // Consuming the '{' token
        self.consume(Token::OpenBrace, "Expected '{' before function body.")?;
    
        // Parsing the function body
        let mut body = Vec::new();
        while self.peek().token_type != Token::CloseBrace && !self.is_at_end() {
            body.push(self.parse_statement()?);
        }
    
        // Consuming the '}' token
        self.consume(Token::CloseBrace, "Expected '}' after function body.")?;
    
        Ok(Stmt::PureFunctionDeclaration {
            name,
            parameters,
            return_type,
            body,
        })
    }
    // Metody pomocnicze do nawigacji po tokenach

    fn advance(&mut self) -> &Token {
        // ...
        unimplemented!()

    }

    fn is_at_end(&self) -> bool {
        // ...
        unimplemented!()

    }

    fn peek(&self) -> &Token {
        // ...
        unimplemented!()

    }

    fn previous(&self) -> &Token {
        // ...
        unimplemented!()

    }
    fn parse_let_declaration(&mut self) -> Result<Stmt, String> {
        // ...
        unimplemented!()

    }

    fn parse_if_statement(&mut self) -> Result<Stmt, String> {
        // ...
        unimplemented!()

    }

    fn parse_while_statement(&mut self) -> Result<Stmt, String> {
        // ...
        unimplemented!()

    }

    fn parse_expression(&mut self) -> Result<Stmt, String> {
        // ...
        unimplemented!()

    }


    fn parse_return_statement(&mut self) -> Result<Stmt, String> {
        // ...
        unimplemented!()

    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        // ...
        unimplemented!()

    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        // ...
        unimplemented!()

    }

    fn parse_binary(&mut self, left: Expr) -> Result<Expr, String> {
        // ...
        unimplemented!()

    }
    fn consume(&mut self, token_type: Token, error_msg: &str) -> Result<Token, String> {
        // ...
        unimplemented!()

    }

    fn check(&self, token_type: Token) -> bool {
        // ...
        unimplemented!()
    }

    fn match_token(&mut self, token_types: &[Token]) -> bool {
        // ...
        unimplemented!()

    }

    // Metody pomocnicze do analizy składniowej

    fn synchronize(&mut self) {
        // ...
        unimplemented!()

    }

    fn precedence(&self, token_type: Token) -> Precedence {
        // ...
        unimplemented!()

    }
    fn parse_parameter(&mut self) -> Result<Parameter, String> {
        let name = self.consume(Token::Identifier("parameter_name".to_string()), "Expected parameter name.")?;
        self.consume(Token::Colon, "Expected ':' after parameter name.")?;
        let param_type = self.parse_type()?;

        Ok(Parameter { name, param_type })
    }


 
    fn parse_type(&mut self) -> Result<Type, String> {
        if self.match_token(&[Token::NumberType]) {
            Ok(Type::Int)
        } else if self.match_token(&[Token::NumberLiteral(NumberLiteral::Float(_))]) {
            Ok(Type::Float)
        } else if self.match_token(&[Token::BoolType]) {
            Ok(Type::Bool)
        } else if self.match_token(&[Token::StringLiteral]) {
            Ok(Type::String)
        } else {
            Err(format!("Expected type, found {:?}", self.peek()))
        }
        
    }


    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        // Zużyj lewy nawias klamrowy '{'
        self.consume(Token::OpenBrace, "Expected '{' at the beginning of a block.")?;
    
        let mut statements = Vec::new();
        
        // Analizuj instrukcje w bloku, dopóki nie napotkasz prawego nawiasu klamrowego '}'
        while self.peek().token_type != Token::OpenBrace && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
    
        // Zużyj prawy nawias klamrowy '}'
        self.consume(Token::CloseBrace, "Expected '}' at the end of a block.")?;
    
        Ok(statements)
    }
}

// Definicja struktur danych dla obsługi błędów i priorytetów operatorów

