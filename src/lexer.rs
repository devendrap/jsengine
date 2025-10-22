// Lexer for tokenizing JavaScript source code
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Number(f64),
    String(String),
    True,
    False,
    Null,
    Undefined,

    // Identifiers and keywords
    Identifier(String),
    Let,
    Const,
    Var,
    Function,
    Return,
    If,
    Else,
    While,
    For,
    Break,
    Continue,
    Throw,
    Try,
    Catch,
    Finally,
    This,
    New,
    InstanceOf,
    In,
    TypeOf,

    // Operators
    Plus,
    PlusPlus,
    Minus,
    MinusMinus,
    Star,
    Slash,
    Percent,
    StarStar,
    Eq,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    BitAndEq,
    BitOrEq,
    BitXorEq,
    ShlEq,
    ShrEq,
    UShrEq,
    AndEq,
    OrEq,
    EqEq,
    EqEqEq,
    Ne,
    NeEq,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Not,
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    Shl,
    Shr,
    UShr,

    // Punctuation
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Semicolon,
    Comma,
    Dot,
    Colon,
    Question,
    Arrow,

    // Special
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::Identifier(s) => write!(f, "{}", s),
            _ => write!(f, "{:?}", self),
        }
    }
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.get(0).copied();
        Lexer {
            input: chars,
            position: 0,
            current_char,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.input.get(self.position + offset).copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        // Single-line comment
        if self.current_char == Some('/') && self.peek(1) == Some('/') {
            while self.current_char.is_some() && self.current_char != Some('\n') {
                self.advance();
            }
            return;
        }

        // Multi-line comment
        if self.current_char == Some('/') && self.peek(1) == Some('*') {
            self.advance(); // skip /
            self.advance(); // skip *
            while self.current_char.is_some() {
                if self.current_char == Some('*') && self.peek(1) == Some('/') {
                    self.advance(); // skip *
                    self.advance(); // skip /
                    break;
                }
                self.advance();
            }
        }
    }

    fn read_number(&mut self) -> f64 {
        let mut num_str = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() || ch == '.' {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        num_str.parse().unwrap_or(0.0)
    }

    fn read_string(&mut self, quote: char) -> String {
        let mut result = String::new();
        self.advance(); // skip opening quote

        while let Some(ch) = self.current_char {
            if ch == quote {
                self.advance(); // skip closing quote
                break;
            } else if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.current_char {
                    let to_push = match escaped {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '\'' => '\'',
                        '"' => '"',
                        _ => escaped,
                    };
                    result.push(to_push);
                    self.advance();
                }
            } else {
                result.push(ch);
                self.advance();
            }
        }

        result
    }

    fn read_identifier(&mut self) -> String {
        let mut result = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' || ch == '$' {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        result
    }

    pub fn next_token(&mut self) -> Token {
        loop {
            self.skip_whitespace();

            // Check for comments
            if self.current_char == Some('/') &&
               (self.peek(1) == Some('/') || self.peek(1) == Some('*')) {
                self.skip_comment();
                continue;
            }

            break;
        }

        let ch = match self.current_char {
            Some(c) => c,
            None => return Token::Eof,
        };

        // Numbers
        if ch.is_ascii_digit() {
            return Token::Number(self.read_number());
        }

        // Strings
        if ch == '"' || ch == '\'' {
            return Token::String(self.read_string(ch));
        }

        // Identifiers and keywords
        if ch.is_alphabetic() || ch == '_' || ch == '$' {
            let ident = self.read_identifier();
            return match ident.as_str() {
                "let" => Token::Let,
                "const" => Token::Const,
                "var" => Token::Var,
                "function" => Token::Function,
                "return" => Token::Return,
                "if" => Token::If,
                "else" => Token::Else,
                "while" => Token::While,
                "for" => Token::For,
                "break" => Token::Break,
                "continue" => Token::Continue,
                "throw" => Token::Throw,
                "try" => Token::Try,
                "catch" => Token::Catch,
                "finally" => Token::Finally,
                "this" => Token::This,
                "new" => Token::New,
                "instanceof" => Token::InstanceOf,
                "in" => Token::In,
                "true" => Token::True,
                "false" => Token::False,
                "null" => Token::Null,
                "undefined" => Token::Undefined,
                "typeof" => Token::TypeOf,
                _ => Token::Identifier(ident),
            };
        }

        // Multi-character operators
        let token = match ch {
            '+' => {
                self.advance();
                if self.current_char == Some('+') {
                    self.advance();
                    Token::PlusPlus
                } else if self.current_char == Some('=') {
                    self.advance();
                    Token::PlusEq
                } else {
                    Token::Plus
                }
            }
            '-' => {
                self.advance();
                if self.current_char == Some('-') {
                    self.advance();
                    Token::MinusMinus
                } else if self.current_char == Some('=') {
                    self.advance();
                    Token::MinusEq
                } else {
                    Token::Minus
                }
            }
            '*' => {
                self.advance();
                if self.current_char == Some('*') {
                    self.advance();
                    Token::StarStar
                } else if self.current_char == Some('=') {
                    self.advance();
                    Token::StarEq
                } else {
                    Token::Star
                }
            }
            '/' => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::SlashEq
                } else {
                    Token::Slash
                }
            }
            '%' => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::PercentEq
                } else {
                    Token::Percent
                }
            }
            '=' => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        Token::EqEqEq
                    } else {
                        Token::EqEq
                    }
                } else if self.current_char == Some('>') {
                    self.advance();
                    Token::Arrow
                } else {
                    Token::Eq
                }
            }
            '!' => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        Token::NeEq
                    } else {
                        Token::Ne
                    }
                } else {
                    Token::Not
                }
            }
            '<' => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::Le
                } else if self.current_char == Some('<') {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        Token::ShlEq
                    } else {
                        Token::Shl
                    }
                } else {
                    Token::Lt
                }
            }
            '>' => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::Ge
                } else if self.current_char == Some('>') {
                    self.advance();
                    if self.current_char == Some('>') {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            Token::UShrEq
                        } else {
                            Token::UShr
                        }
                    } else if self.current_char == Some('=') {
                        self.advance();
                        Token::ShrEq
                    } else {
                        Token::Shr
                    }
                } else {
                    Token::Gt
                }
            }
            '&' => {
                self.advance();
                if self.current_char == Some('&') {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        Token::AndEq
                    } else {
                        Token::And
                    }
                } else if self.current_char == Some('=') {
                    self.advance();
                    Token::BitAndEq
                } else {
                    Token::BitAnd
                }
            }
            '|' => {
                self.advance();
                if self.current_char == Some('|') {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        Token::OrEq
                    } else {
                        Token::Or
                    }
                } else if self.current_char == Some('=') {
                    self.advance();
                    Token::BitOrEq
                } else {
                    Token::BitOr
                }
            }
            '^' => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::BitXorEq
                } else {
                    Token::BitXor
                }
            }
            '~' => {
                self.advance();
                Token::BitNot
            }
            '(' => {
                self.advance();
                Token::LParen
            }
            ')' => {
                self.advance();
                Token::RParen
            }
            '{' => {
                self.advance();
                Token::LBrace
            }
            '}' => {
                self.advance();
                Token::RBrace
            }
            '[' => {
                self.advance();
                Token::LBracket
            }
            ']' => {
                self.advance();
                Token::RBracket
            }
            ';' => {
                self.advance();
                Token::Semicolon
            }
            ',' => {
                self.advance();
                Token::Comma
            }
            '.' => {
                self.advance();
                Token::Dot
            }
            ':' => {
                self.advance();
                Token::Colon
            }
            '?' => {
                self.advance();
                Token::Question
            }
            _ => {
                self.advance();
                return self.next_token();
            }
        };

        token
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        tokens
    }
}
