use logos::Logos;
use num_bigint::BigInt;

#[derive(Debug, PartialEq)]
pub enum NumberLiteral {
    Int(i64),
    Float(f64),
    BigInt(BigInt),
}

fn parse_identifier(lex: &mut logos::Lexer<Token>) -> String {
    lex.slice().to_string()
}

fn parse_number_literal(lex: &mut logos::Lexer<Token>) -> NumberLiteral {
    let slice = lex.slice();
    if let Ok(int_value) = slice.parse::<i64>() {
        NumberLiteral::Int(int_value)
    } else if let Ok(float_value) = slice.parse::<f64>() {
        NumberLiteral::Float(float_value)
    } else if let Ok(big_int_value) = slice.parse::<BigInt>() {
        NumberLiteral::BigInt(big_int_value)
    } else {
        panic!("Failed to parse number literal")
    }
}


fn parse_string_literal(lex: &mut logos::Lexer<Token>) -> String {
    let slice = lex.slice();
    let content = &slice[1..slice.len() - 1];
    content.replace("\\\"", "\"").replace("\\'", "'")
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {

    #[token("variant")]
    Variant,

    #[token("TokenType")]
    TokenType,

    #[token("end")]
    End,

    #[token("let")]
    Let,

    #[token("if")]
    If,

    #[token("while")]
    While,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("return")]
    Return,

    #[token("Number")]
    NumberType,

    #[token("=>")]
    ReturnArrow,

    #[token("{")]
    OpenBrace,

    #[token("}")]
    CloseBrace,

    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token(":")]
    Colon,

    #[token(";")]
    Semicolon,

    #[token(",")]
    Comma,

    #[token("+")]
    Plus,

    #[regex("//[^\n]*\n?")]
    SingleLineComment,

    #[regex("/\\*[^*]*\\*+([^/*][^*]*\\*+)*/")]
    MultiLineComment,

    #[token("=")]
    Equals,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", callback = parse_identifier)]
    Identifier(String),

    #[regex("\"(\\\\.|[^\"])*\"|'(\\\\.|[^'])*'", callback = parse_string_literal)]
    StringLiteral(String),

    #[regex(r"(\d+(\.\d*)?|\.\d+)([eE][+-]?\d+)?", callback = parse_number_literal)]
    NumberLiteral(NumberLiteral),

    #[token("pf")]
    PureFunctionKeyword,

    #[token("io")]
    IdempotentOperationKeyword,

    #[token("cd")]
    ConstraintDeclarationKeyword,

    #[token("lc")]
    LayeredCompositionKeyword,

    #[token("pm")]
    PatternMatchingKeyword,

    #[token("rf")]
    RecursiveFunctionKeyword,

    #[token("nf")]
    NestedFunctionKeyword,

    #[token("st")]
    StrongTypingKeyword,

    #[token("le")]
    LazyEvaluationKeyword,

    #[token("ol")]
    OptimizationLibraryKeyword,

    #[token("ch")]
    ConstraintHandlingKeyword,

    #[token("cs")]
    ConcurrencySupportKeyword,

    #[regex("[ \t\n\r]+", logos::skip)]
    Whitespace,
    
    #[error]
    Error,
}


pub fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut lexer = Token::lexer(code);

    while let Some(token) = lexer.next() {
        match token {
            Token::MultiLineComment => {
                let _stripped = lexer.slice()[2..lexer.slice().len() - 2].trim();
                tokens.push(Token::MultiLineComment); // Opcjonalnie, możemy dodać zmodyfikowany token
            }
            _ => tokens.push(token),
        }
    }

    tokens
}


