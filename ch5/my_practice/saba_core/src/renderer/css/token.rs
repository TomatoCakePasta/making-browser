use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub enum CssToken {
    HashToken(String),
    Delim(char),
    Number(f64),
    Colon,
    SemiColon,
    OpenParenthesis,
    CloseParenthesis,
    OpenCurly,
    CloseCurly,
    Indent(String),
    StringToken(String),
    AtKeyword(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CssTokenizer {
    pos: usize,
    input: Vec<char>,
}

impl CssTokenizer {
    // constructor
    pub fn new(css: String) -> Self {
        Self  {
            pos: 0,
            // chars() extracts characters one by one.
            // collect() collects them into a different type.
            // e.g.
            // "Rust" -> ['R', 'u', 's', 't']
            input: css.chars().collect(),
        }
    }

    // consume_string_token() interprets the input as characters until another " or ' is encountered
    fn consume_string_token(&mut self) -> String {
        let mut s = String::new();

        loop {
            if self.pos >= self.input.len() {
                return s;
            }

            self.pos += 1;
            let c = self.input[self.pos];
            match c {
                '"' | '\'' => break,
                _ => s.push(c),
            }
        }

        s
    }

    // As long as a number or period continues to appear, it is interpreted as a number.
    fn consume_numeric_token(&mut self) -> f64 {
        let mut num = 0f64;
        let mut floating = false;
        let mut floating_digit = 1f64;

        loop {
            if self.pos >= self.input.len() {
                return num;
            }

            let c = self.input[self.pos];

            match c {
                '0'..='9' => {
                    if floating {
                        floating_digit *= 1f64 / 10f64;
                        num += (c.to_digit(10).unwrap() as f64) * floating_digit;
                    } else {
                        // num * 10.0 Carry forward
                        num = num * 10.0 + (c.to_digit(10).unwrap() as f64);
                    }
                    self.pos += 1;
                }
                '.' => {
                    floating = true;
                    self.pos += 1;
                }
                _ => break,
            }
        }

        num
    }

    // As long as a character, numberm, '-' or '_' continues to appear, it is interpreted as a identifier.
    fn consume_indent_token(&mut self) -> String {
        let mut s = String::new();
        s.push(self.input[self.pos]);

        loop {
            self.pos += 1;
            let c = self.input[self.pos];
            
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => {
                    s.push(c);
                }
                _ => break,
            }
        }

        s
    }
}

// Iterators define common rules
impl Iterator for CssTokenizer {
    type Item = CssToken;

    // return next token
    // Check the CSS string character by character
    fn next(&mut self) -> Option<Self::Item> {
        // check 
        loop {
            if self.pos >= self.input.len() {
                return None;
            }

            let c = self.input[self.pos];

            let token = match c {
                // Decide next token
                '(' => CssToken::OpenParenthesis,
                ')' => CssToken::CloseParenthesis,
                ',' => CssToken::Delim(','),
                '.' => CssToken::Delim('.'),
                ':' => CssToken::Colon,
                ';' => CssToken::SemiColon,
                '{' => CssToken::OpenCurly,
                '}' => CssToken::CloseCurly,
                ' ' | '\n' => {
                    self.pos += 1;
                    continue;
                }
                '"' | '\'' => {
                    // consume_string_token() interprets the input as characters until another " or ' is encountered
                    // e.g.
                    // p { content: \"Hey\"; }
                    // value = Hey
                    let value = self.consume_string_token();
                    CssToken::StringToken(value)
                }
                '0'..='9' => {
                    let t = CssToken::Number(self.consume_numeric_token());
                    self.pos -= 1;
                    t
                }
                '#' => {
                    // This time, it will always be treated as an ID selector in the format #ID
                    let value = self.consume_indent_token();
                    self.pos -= 1;
                    CssToken::HashToken(value)
                }
                '-' => {
                    // This book does not deal with negative numbers, 
                    // so the hyphen is treated as an identifier.
                    let t = CssToken::Indent(self.consume_indent_token());
                    self.pos -= 1;
                    t
                }
                '@' => {
                    // If the next three characters are valid identifier characters,  <at-keyword-token>
                    // create and return a token.
                    // otherwise, return <delim-token>

                    // What we want to know is whether the @ appears to be followed by an identifier.
                    if self.input[self.pos + 1].is_ascii_alphabetic()
                        && self.input[self.pos + 2].is_alphanumeric()
                        && self.input[self.pos + 3].is_alphanumeric()
                    {
                        // skip '@'
                        self.pos += 1;
                        let t = CssToken::AtKeyword(self.consume_indent_token());
                        self.pos -= 1;
                        t
                    } else {
                        CssToken::Delim('@')
                    }
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let t = CssToken::Indent(self.consume_indent_token());
                    self.pos -= 1;
                    t
                }
                _ => {
                    unimplemented!("char {} is not supported yet", c);
                }
            };

            self.pos += 1;
            return Some(token);
        }
    }
}
