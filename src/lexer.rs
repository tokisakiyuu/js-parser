use lexgen::lexer;
use std::str::FromStr;

lexer! {
    pub Lexer -> Token<'input>;
    type Error = String;

    rule Init {
        [' ' '\t' '\n']+,

        "var" = Token::Var,
        "let" = Token::Let,
        "const" = Token::Const,
        "function" = Token::Function,
        "(" = Token::LeftParen,
        ")" = Token::RightParen,
        "{" = Token::LeftCurlyBrace,
        "}" = Token::RightCurlyBrace,
        "=" = Token::Assign,
        ";" = Token::Semicolon,
        "." = Token::Dot,
        "," = Token::Comma,

        $$XID_Start $$XID_Continue* => |lexer| {
            let id = lexer.match_();
            lexer.return_(Token::Identifier(id))
        },

        ['0'-'9']+ =? |lexer| {
            let s = lexer.match_();
            match f64::from_str(s) {
                Ok(num) => lexer.return_(Ok(Token::Number(num))),
                Err(_) => lexer.return_(Err("invalid number".to_string())),
            }
        },

        ['0'-'9']+ '.' ['0'-'9']+ =? |lexer| {
            let s = lexer.match_();
            match f64::from_str(s) {
                Ok(num) => lexer.return_(Ok(Token::Number(num))),
                Err(_) => lexer.return_(Err("invalid number".to_string())),
            }
        },

        '.' ['0'-'9']+ =? |lexer| {
            let s = lexer.match_();
            match f64::from_str(s) {
                Ok(num) => lexer.return_(Ok(Token::Number(num))),
                Err(_) => lexer.return_(Err("invalid number".to_string())),
            }
        },

        '"' => |lexer| lexer.switch(LexerRule::String),
        '\'' => |lexer| lexer.switch(LexerRule::SingleQuoteString),
    },

    rule String {
        '"' => |lexer| {
            let match_ = lexer.match_();
            lexer.switch_and_return(LexerRule::Init, Token::StringLiteral(&match_[1..match_.len() - 1]))
        },
        _ => |lexer| lexer.continue_(),
    },

    rule SingleQuoteString {
        '\'' => |lexer| {
            let match_ = lexer.match_();
            lexer.switch_and_return(LexerRule::Init, Token::StringLiteral(&match_[1..match_.len() - 1]))
        },
        _ => |lexer| lexer.continue_(),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token<'input> {
    Identifier(&'input str),
    StringLiteral(&'input str),
    Number(f64),

    Var,
    Let,
    Const,
    Function,

    /// "("
    LeftParen,
    /// ")"
    RightParen,
    /// "{"
    LeftCurlyBrace,
    /// "}"
    RightCurlyBrace,
    /// "="
    Assign,
    /// ";"
    Semicolon,
    /// "."
    Dot,
    /// ","
    Comma,
}

impl<'input> Into<f64> for Token<'input> {
    fn into(self) -> f64 {
        use Token::*;
        match self {
            Number(n) => n,
            _ => unimplemented!(),
        }
    }
}

impl<'input> Into<&'input str> for Token<'input> {
    fn into(self) -> &'input str {
        use Token::*;
        match self {
            StringLiteral(s) => s,
            Identifier(id) => id,
            _ => unimplemented!(),
        }
    }
}

pub type LexerError = lexgen_util::LexerError<String>;

#[test]
fn test_lexer() {
    use lexgen_util::Loc;
    use Token::*;

    let lexer =
        Lexer::new("let const var function hello ( ) { } = , ; 23.45 67 .89 'halo' \"nihao\"");
    let tokens: Vec<Result<(Loc, Token, Loc), LexerError>> = lexer.collect();
    let token_types: Vec<Token> = tokens.into_iter().map(|r| r.unwrap().1).collect();
    assert_eq!(
        token_types,
        vec![
            Let,
            Const,
            Var,
            Function,
            Identifier("hello"),
            LeftParen,
            RightParen,
            LeftCurlyBrace,
            RightCurlyBrace,
            Assign,
            Comma,
            Semicolon,
            Number(23.45),
            Number(67 as f64),
            Number(0.89),
            StringLiteral("halo"),
            StringLiteral("nihao")
        ]
    );
}
