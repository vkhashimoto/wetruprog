use super::token_type::TokenType;
use std::{iter::Peekable, str::Chars};

type Char = Option<char>;

struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    ch: Char,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer {
            source: source.chars().peekable(),
            ch: None,
        };

        lexer.advance_char();

        lexer
    }

    pub fn next_token(&mut self) -> TokenType {
        self.advance_ch_until(|c| !c.is_whitespace());
        // used to advance or not advance after reading the token
        let mut advance_next_char = Some(Box::new(|l: &mut Lexer| {
            l.advance_next_char_until(|c| !c.is_whitespace())
        }));
        let token_type = match self.ch {
            Some('0'..='9') => {
                let token_type = self.read_integer();
                advance_next_char = None;
                token_type
            },
            Some('+') => TokenType::Plus,
            Some('-') => TokenType::Minus,
            Some('*') => TokenType::Asterisk,
            Some('/') => TokenType::Slash,
            Some('(') => TokenType::LParen,
            Some(')') => TokenType::RParen,
            _ => TokenType::Eof,
        };

        match advance_next_char {
            Some(func) => func(self),
            _ => {}
        };
        return token_type;
    }

    fn advance_char(&mut self) {
        self.ch = self.source.next();
    }

    fn next_char(&mut self) -> Char {
        self.advance_char();
        self.ch
    }

    fn advance_next_char_until(&mut self, filter: fn(char) -> bool) {
        while let Some(cch) = self.next_char() {
            if filter(cch) {
                break;
            }
        }
    }

    fn advance_ch_until(&mut self, filter: fn(char) -> bool) {
        while let Some(cch) = self.ch {
            if filter(cch) {
                break;
            }
            self.advance_char();
        }
    }

    fn read_integer(&mut self) -> TokenType {
        let integer = self.read_chars_until(|c| !c.is_digit(10));

        return TokenType::Integer(
            integer
                .parse()
                .expect(&format!("Failed to convert {} to integer.", integer)),
        );
    }

    fn read_chars_until(&mut self, filter: fn(char) -> bool) -> String {
        let mut string = String::new();
        string.push(self.ch.unwrap());

        while let Some(next_char) = self.next_char() {
            if filter(next_char) {
                break;
            }

            string.push(next_char);
        }

        return string;
    }
}

#[cfg(test)]
mod tests {
    use crate::lang::token_type::TokenType;

    use super::Lexer;

    #[test]
    fn lex_expressions() {
        let source = r#"1 + 1
1 - 2
101 * 2101
1 / 2
(1 + 2) * 2"#;

        let mut lexer = Lexer::new(source);
        assert_eq!(lexer.next_token(), TokenType::Integer(1));
        assert_eq!(lexer.next_token(), TokenType::Plus);
        assert_eq!(lexer.next_token(), TokenType::Integer(1));

        assert_eq!(lexer.next_token(), TokenType::Integer(1));
        assert_eq!(lexer.next_token(), TokenType::Minus);
        assert_eq!(lexer.next_token(), TokenType::Integer(2));

        assert_eq!(lexer.next_token(), TokenType::Integer(101));
        assert_eq!(lexer.next_token(), TokenType::Asterisk);
        assert_eq!(lexer.next_token(), TokenType::Integer(2101));

        assert_eq!(lexer.next_token(), TokenType::Integer(1));
        assert_eq!(lexer.next_token(), TokenType::Slash);
        assert_eq!(lexer.next_token(), TokenType::Integer(2));

        assert_eq!(lexer.next_token(), TokenType::LParen);
        assert_eq!(lexer.next_token(), TokenType::Integer(1));
        assert_eq!(lexer.next_token(), TokenType::Plus);
        assert_eq!(lexer.next_token(), TokenType::Integer(2));
        assert_eq!(lexer.next_token(), TokenType::RParen);
        assert_eq!(lexer.next_token(), TokenType::Asterisk);
        assert_eq!(lexer.next_token(), TokenType::Integer(2));
    }
}
