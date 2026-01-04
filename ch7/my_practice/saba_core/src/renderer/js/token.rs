use alloc::string::String;
use alloc::vec::Vec;
use alloc::string::ToString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Punctuator(char),
    Number(u64),
}

// Lexer is similar to tokenizer
pub struct JsLexer {
    pos: usize,
    input: Vec<char>,
}

impl JsLexer {
    // constuctor
    pub fn new(js: String) -> Self {
        Self {
            pos: 0,
            input: js.chars().collect(),
        }
    }

    // As long as numbers continue to appear, 
    // consume characters and interpret them as numbers.
    fn consume_number(&mut self) -> u64 {
        let mut num = 0;

        loop {
            if self.pos >= self.input.len() {
                return num;
            }

            let c = self.input[self.pos];

            match c {
                '0'..='9' => {
                    num = num * 10 + (c.to_digit(10).unwrap() as u64);
                    self.pos += 1;
                }
                _ => break,
            }
        }

        return num;
    }
}

impl Iterator for JsLexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // return token
        if self.pos >= self.input.len() {
            return None;
        }

        // Advance to the next position as long as there is a space or newline
        while self.input[self.pos] == ' ' || self.input[self.pos] == '\n' {
            self.pos += 1;

            if self.pos >= self.input.len() {
                return None;
            }
        }

        let c = self.input[self.pos];

        let token = match c {
            '+' | '-' | ';' | '=' | '(' | ')' | '{' | '}' | ',' | '.' => {
                let t = Token::Punctuator(c);

                self.pos += 1;
                t
            }
            // As long as numbers continue to appear, 
            // consume characters and interpret them as numbers.
            '0'..='9' => Token::Number(self.consume_number()),
            _ => unimplemented!("char {:?} is not supported yet", c),
        };

        Some(token)
    }
}