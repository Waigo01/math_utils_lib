#[cfg(feature = "async")]
use async_macro::function_async;

use async_macro::async_call;

use std::ops::Deref;

use crate::errors::TokenizerError;

#[derive(Clone, Debug, PartialEq)]
pub enum Delimiter {
    Parenthesis,
    Brace,
    Bracket
}

impl Delimiter {
    pub fn to_char(&self) -> char {
        match self {
            Self::Brace => '{',
            Self::Bracket => '[',
            Self::Parenthesis => '('
        }
    }
}

#[derive(Clone, Debug)]
pub struct Group {
    pub delimiter: Delimiter,
    pub stream: TokenStream
}

#[derive(Clone, Debug)]
pub enum Token {
    Group(Group),
    Ident(String),
    Punct(char),
    Literal(String)
}

impl Token {
    pub fn is_group(&self) -> bool {
        match self {
            Token::Group(_) => true,
            _ => false
        }
    }
    pub fn is_ident(&self) -> bool {
        match self {
            Token::Ident(_) => true,
            _ => false
        }
    }
    pub fn is_punct(&self) -> bool {
        match self {
            Token::Punct(_) => true,
            _ => false
        }
    }
    pub fn is_literal(&self) -> bool {
        match self {
            Token::Literal(_) => true,
            _ => false
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Token::Literal(l) => l.to_string(),
            Token::Ident(i) => i.to_string(),
            Token::Punct(p) => p.to_string(),
            Token::Group(group) => group.stream.to_string()
        }
    }
}

#[derive(Clone, Debug)]
pub struct TokenStream(Vec<Box<Token>>);

impl TokenStream {
    pub fn new() -> Self {
        TokenStream(vec![])
    }
    pub fn from_tokens(tokens: Vec<Box<Token>>) -> Self {
        TokenStream(tokens)
    }
    pub fn to_string(&self) -> String {
        self.0.iter().map(|t| t.to_string()).collect()
    }
    pub fn push(&mut self, token: Token) {
        self.0.push(Box::new(token));
    }
}

impl Deref for TokenStream {
  type Target = [Box<Token>];

  fn deref(&self) -> &[Box<Token>] {
    &self.0[..]
  }
}

impl IntoIterator for TokenStream {
  type Item = Box<Token>;
  type IntoIter = <Vec<Box<Token>> as IntoIterator>::IntoIter;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

pub struct Tokenizer {
    state: u32,
    stream: TokenStream,
    ident_buffer: String,
    groups_buffer: Vec<Group>,
    literal_buffer: String,
}

const PUNCTS: [char; 14] = ['@', '-', '*', '/', '#', ',', '&', '|', '+', '=', '!', '<', '>', '^'];

impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer { state: 0, stream: TokenStream::new(), ident_buffer: String::new(), groups_buffer: vec![], literal_buffer: String::new() }
    }

    #[cfg_attr(feature = "async", function_async)]
    pub fn tokenize<S: Into<String>>(&mut self, expr: S) -> Result<TokenStream, TokenizerError> {
        let whitespaced_string: String = expr.into().trim().split(" ").filter(|s| !s.is_empty()).collect();
        let whitespaced_string = whitespaced_string.replace("\n", "");
        let expr_chars = whitespaced_string.chars();

        for ch in expr_chars {
            async_call!(self.read_char(ch))?;
        }

        if !self.ident_buffer.is_empty() {
            self.stream.push(Token::Ident(self.ident_buffer.clone()));
        } else if !self.literal_buffer.is_empty() {
            self.stream.push(Token::Literal(self.literal_buffer.clone()));
        }

        if self.groups_buffer.len() != 0 {
            return Err(TokenizerError::UnmatchedDelimiter);
        }

        return Ok(self.stream.clone());
    }

    #[cfg_attr(feature = "async", function_async)]
    fn push_token(&mut self, token: Token) {
        if self.groups_buffer.len() != 0 {
            self.groups_buffer.last_mut().unwrap().stream.push(token);
        } else {
            self.stream.push(token);
        }
    }

    #[cfg_attr(feature = "async", function_async)]
    fn read_char(&mut self, ch: char) -> Result<(), TokenizerError> {
        match self.state {
            0 if ch.is_alphabetic() || ch == '\\' => {self.ident_buffer.push(ch); self.state = 1},
            1 if !ch.is_alphabetic() && ch != '_' => {
                if self.ident_buffer.is_empty() || self.ident_buffer == "\\" {return Err(TokenizerError::InvalidIdentName)}
                async_call!(self.push_token(Token::Ident(self.ident_buffer.clone())));
                self.ident_buffer = String::new();
                self.state = 0;
                async_call!(self.read_char(ch))?;
            },
            1 if ch == '_' => {self.ident_buffer.push(ch); self.state = 2;},
            1 if ch.is_alphabetic() => self.ident_buffer.push(ch),
            2 if ch != '{' => {
                if !ch.is_alphanumeric() {return Err(TokenizerError::InvalidIdentName)}
                self.ident_buffer.push(ch);
                async_call!(self.push_token(Token::Ident(self.ident_buffer.clone())));
                self.ident_buffer = String::new();
                self.state = 0;
            },
            2 if ch == '{' => {self.ident_buffer.push(ch); self.state = 3;},
            3 if ch != '}' => self.ident_buffer.push(ch),
            3 if ch == '}' => {
                self.ident_buffer.push(ch);
                async_call!(self.push_token(Token::Ident(self.ident_buffer.clone())));
                self.ident_buffer = String::new();
                self.state = 0;
            },
            0 if PUNCTS.contains(&ch) => self.push_token(Token::Punct(ch)),
            0 if ch.is_numeric() => {self.literal_buffer.push(ch); self.state = 7},
            7 if ch.is_numeric() || ch == '.' => self.literal_buffer.push(ch),
            7 => {
                if self.literal_buffer.len() == 0 || self.literal_buffer.ends_with(".") {return Err(TokenizerError::InvalidLiteral(self.literal_buffer.clone()))}
                async_call!(self.push_token(Token::Literal(self.literal_buffer.clone())));
                self.literal_buffer = String::new();
                self.state = 0;
                async_call!(self.read_char(ch))?;
            },

            0 if ch == '{' => self.groups_buffer.push(Group { delimiter: Delimiter::Brace, stream: TokenStream::new() }),
            0 if ch == '(' => self.groups_buffer.push(Group { delimiter: Delimiter::Parenthesis, stream: TokenStream::new() }),
            0 if ch == '[' => self.groups_buffer.push(Group { delimiter: Delimiter::Bracket, stream: TokenStream::new() }),
            0 if ch == '}' => {
                if self.groups_buffer.len() == 0 || self.groups_buffer.last().unwrap().delimiter != Delimiter::Brace {return Err(TokenizerError::UnmatchedDelimiter)}
                let group = self.groups_buffer.remove(self.groups_buffer.len()-1);
                async_call!(self.push_token(Token::Group(group)));
            },
            0 if ch == ')' => { 
                if self.groups_buffer.len() == 0 || self.groups_buffer.last().unwrap().delimiter != Delimiter::Parenthesis {return Err(TokenizerError::UnmatchedDelimiter)}
                let group = self.groups_buffer.remove(self.groups_buffer.len()-1);
                async_call!(self.push_token(Token::Group(group)));
            },
            0 if ch == ']' => { 
                if self.groups_buffer.len() == 0 || self.groups_buffer.last().unwrap().delimiter != Delimiter::Bracket {return Err(TokenizerError::UnmatchedDelimiter)}
                let group = self.groups_buffer.remove(self.groups_buffer.len()-1);
                async_call!(self.push_token(Token::Group(group)));
            },

            0 if !ch.is_alphanumeric() => return Err(TokenizerError::InvalidPunct(ch)),

            _ => {return Err(TokenizerError::InvalidState)}
        }

        Ok(())
    }
}
