#[cfg(test)]
mod tests {

    use aig_compiler::compiler::lexer::{Token, tokenize, NumberLiteral, Token::StringLiteral};
    
    // Test for single token
    #[test]
    fn test_end_token() {
        let code = "end";
        let expected_tokens = vec![Token::End];
        assert_eq!(tokenize(code), expected_tokens);
    }

    // Test for multiple tokens
    #[test]
    fn test_complex_code() {
        let code = "let x: Number = 123 + 456;";
        let expected_tokens = vec![
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Colon,
            Token::NumberType,
            Token::Equals,
            aig_compiler::compiler::lexer::Token::NumberLiteral(NumberLiteral::Int(123)),
            Token::Plus,
            aig_compiler::compiler::lexer::Token::NumberLiteral(NumberLiteral::Int(456)),
            Token::Semicolon,
        ];
        assert_eq!(tokenize(code), expected_tokens);
    }

    // Test for keywords
    #[test]
    fn test_keyword_tokens() {
        let code = "let x = pf(y) io(z) cd(a) lc(b) pm(c) rf(d) nf(e) st(f) le(g) ol(h) ch(i) cs(j);";
        let expected_tokens = vec![
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Equals,
            Token::PureFunctionKeyword,
            Token::OpenParen,
            Token::Identifier("y".to_ascii_lowercase()),
            Token::CloseParen,
            Token::IdempotentOperationKeyword,
            Token::OpenParen,
            Token::Identifier("z".to_ascii_lowercase()),
            Token::CloseParen,
            Token::ConstraintDeclarationKeyword,
            Token::OpenParen,
            Token::Identifier("a".to_ascii_lowercase()),
            Token::CloseParen,
            Token::LayeredCompositionKeyword,
            Token::OpenParen,
            Token::Identifier("b".to_ascii_lowercase()),
            Token::CloseParen,
            Token::PatternMatchingKeyword,
            Token::OpenParen,
            Token::Identifier("c".to_ascii_lowercase()),
            Token::CloseParen,
            Token::RecursiveFunctionKeyword,
            Token::OpenParen,
            Token::Identifier("d".to_ascii_lowercase()),
            Token::CloseParen,
            Token::NestedFunctionKeyword,
            Token::OpenParen,
            Token::Identifier("e".to_ascii_lowercase()),
            Token::CloseParen,
            Token::StrongTypingKeyword,
            Token::OpenParen,
            Token::Identifier("f".to_ascii_lowercase()),
            Token::CloseParen,
            Token::LazyEvaluationKeyword,
            Token::OpenParen,
            Token::Identifier("g".to_ascii_lowercase()),
            Token::CloseParen,
            Token::OptimizationLibraryKeyword,
            Token::OpenParen,
            Token::Identifier("h".to_ascii_lowercase()),
            Token::CloseParen,
            Token::ConstraintHandlingKeyword,
            Token::OpenParen,
            Token::Identifier("i".to_ascii_lowercase()),
            Token::CloseParen,
            Token::ConcurrencySupportKeyword,
            Token::OpenParen,
            Token::Identifier("j".to_ascii_lowercase()),
            Token::CloseParen,
            Token::Semicolon,
        ];
        assert_eq!(tokenize(code), expected_tokens);
    }

    #[test]
fn test_all_tokens() {
    let code = "end let return Number => { } ( ) : , + = identifier123 123 pf io cd lc pm rf nf st le ol ch cs";
    let expected_tokens = vec![
        Token::End,
        Token::Let,
        Token::Return,
        Token::NumberType,
        Token::ReturnArrow,
        Token::OpenBrace,
        Token::CloseBrace,
        Token::OpenParen,
        Token::CloseParen,
        Token::Colon,
        Token::Comma,
        Token::Plus,
        Token::Equals,
        Token::Identifier("identifier123".to_string()),
        aig_compiler::compiler::lexer::Token::NumberLiteral(NumberLiteral::Int(123)),
        Token::PureFunctionKeyword,
        Token::IdempotentOperationKeyword,
        Token::ConstraintDeclarationKeyword,
        Token::LayeredCompositionKeyword,
        Token::PatternMatchingKeyword,
        Token::RecursiveFunctionKeyword,
        Token::NestedFunctionKeyword,
        Token::StrongTypingKeyword,
        Token::LazyEvaluationKeyword,
        Token::OptimizationLibraryKeyword,
        Token::ConstraintHandlingKeyword,
        Token::ConcurrencySupportKeyword,
    ];
    assert_eq!(tokenize(code), expected_tokens);
}


    #[test]
fn test_single_line_comment_tokenization() {
    let code = "// This is a single line comment \n";
    let expected_tokens = vec![
        Token::SingleLineComment,
    ];
    assert_eq!(tokenize(code), expected_tokens);
}

#[test]
fn test_multi_line_comment_tokenization() {
    let code = "/* This is a \nmulti-line comment */";
    let expected_tokens = vec![
        Token::MultiLineComment,
    ];
    assert_eq!(tokenize(code), expected_tokens);
}

#[test]
fn test_string_literal_tokenization() {
    let code = r#"'single_quotes' "double_quotes""#;
    let expected_tokens = vec![
        Token::StringLiteral("single_quotes".to_string()),
        Token::StringLiteral("double_quotes".to_string()),
    ];
    assert_eq!(tokenize(code), expected_tokens);
}

#[test]
fn test_number_literal_tokenization() {
    let code = "42 3.14 1e-3";
    let expected_tokens = vec![
        Token::NumberLiteral(NumberLiteral::Int(42)),
        Token::NumberLiteral(NumberLiteral::Float(3.14)),
        Token::NumberLiteral(NumberLiteral::Float(1e-3)),
    ];
    assert_eq!(tokenize(code), expected_tokens);
}

#[test]
fn test_identifier_tokenization() {
    let code = "variable _private_var StructName";
    let expected_tokens = vec![
        Token::Identifier("variable".to_string()),
        Token::Identifier("_private_var".to_string()),
        Token::Identifier("StructName".to_string()),
    ];
    assert_eq!(tokenize(code), expected_tokens);
}

}