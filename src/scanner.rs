use crate::token_type::{Literal, Token, TokenType};

struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

pub enum ParsingError {
    UnexpectedEOF(usize),
    UnexpectedChar(char, usize),
}
impl Scanner {
    fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    fn next(&mut self) -> char {
        let a = self.source[self.current];
        self.current += 1;
        a
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> char {
        if self.is_at_end() { '\0' } else { self.source[self.current] }
    }

    fn next_match(&mut self, c: char) -> bool {
        if self.peek() == c {
            self.current += 1;
            true
        }
        else {
            false
        }
    }

    fn token(&mut self, token: TokenType) {
        self.token_with_literal(token, None)
    }

    fn token_with_literal(&mut self, token: TokenType, literal: Option<Literal>) {
        let to_lex: String = self.source[self.start..self.current].iter().collect();
        let t = Token {
            token_type: token,
            literal,
            line: self.line,
            lexeme: to_lex
        };
        self.tokens.push(t);
    }

    fn string(&mut self) -> Result<(), ParsingError> {
        //todo Make string parsing better, should be able to handle double quotes
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1; }
            self.next();
        }

        if self.is_at_end() {
            return Err(ParsingError::UnexpectedEOF(self.line));
        }

        self.next(); // consume the ending quote

        let value: String = self.source[self.start + 1..self.current - 1].iter().collect();
        Ok(self.token_with_literal(TokenType::String, Some(Literal::Str(value))))
    }
    fn next_token(&mut self) -> Result<(), ParsingError> {
        let c = self.next();
        let mut err: Option<ParsingError> = None;
        match c {
            '(' => self.token(TokenType::LeftParen),
            ')' => self.token(TokenType::RightParen),
            '{' => self.token(TokenType::LeftBrace),
            '}' => self.token(TokenType::RightBrace),
            ',' => self.token(TokenType::Comma),
            '.' => self.token(TokenType::Dot),
            '-' => self.token(TokenType::Minus),
            '+' => self.token(TokenType::Plus),
            ';' => self.token(TokenType::Semicolon),
            '*' => self.token(TokenType::Star),
            '<' => {
                if self.next_match('=')
                    { self.token(TokenType::LessEqual) }
                else {
                    self.token(TokenType::Less)
                }
            }
            '>' => {
                if self.next_match('=')
                    { self.token(TokenType::GreaterEqual) }
                else {
                    self.token(TokenType::Greater)
                }
            }
            '!' => {
                if self.next_match('=')
                    { self.token(TokenType::BangEqual) }
                else {
                    self.token(TokenType::Bang)
                }
            }
            '=' => {
                if self.next_match('=')
                    { self.token(TokenType::EqualEqual) }
                else {
                    self.token(TokenType::Equal)
                }
            }
            '/' => {
                if self.next_match('/')
                    { while self.peek() != '\n' && !self.is_at_end() { self.next(); } }
                else {
                    self.token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => { self.line += 1; }
            '"' => {
                if let Err(e) = self.string() {
                    err = Some(e);
                }
            },
            _ => {err = Some(ParsingError::UnexpectedChar(c, self.line));}
        };
        self.start = self.current;
        if let Some(e) = err {
            Err(e)
        } else { Ok(()) }

    }

    fn scan_tokens(&mut self) -> Result<(), Vec<ParsingError>> {
        let mut errors: Vec<ParsingError> = Vec::new();
        while !self.is_at_end() {
            if let Err(e) = self.next_token() {
                errors.push(e);
            }
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::from(""),
            line: self.line,
            literal: None
        });
        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(())
        }

    }
}


///
///
/// # Arguments
///
/// * `source`:
///
/// returns: Result<Vec<Token, Global>, Vec<ParsingError, Global>>
///
/// # Examples
///
/// ```
///
/// ```
// I know this is bad, the scan tokens of the scanner should return the tokens
// but that would require too much refactoring

//todo: Should this also return tokens on error?
pub fn scan_tokens(source: String) -> Result<Vec<Token>, Vec<ParsingError>> {
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens().and(Ok(scanner.tokens))
}