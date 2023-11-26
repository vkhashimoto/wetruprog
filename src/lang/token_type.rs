#[derive(Debug, PartialEq)]
pub enum TokenType {
    Integer(i64),

    Plus,
    Minus,
    Asterisk,
    Slash,

    RParen,
    LParen,

    Eof,
}
