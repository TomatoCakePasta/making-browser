use alloc::string::String;
use alloc::vec::Vec;
use crate::renderer::html::attribute::Attribute;


// State is "how to read" 
// and Token is "what you read"

// Divide String into Tokens
// A token is the smallest meaningful unit of a string
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmlToken {
    // Start tag
    StartTag {
        tag: String,
        // Either independent tag or not
        // e.g. <br />, <img /> is self-closing tag
        // <div>, <p> is not self-closing tags
        self_closing: bool,
        // attributes
        // e.g. <a href="url">, href is attribute
        attributes: Vec<Attribute>,
    },
    // End tag
    EndTag {
        tag: String,
    },
    // String data
    Char(char),
    // End Of File
    Eof,
}

/*

    // basic loop
    Data
    └─ "<" → TagOpen
            ├─ "/" → EndTagOpen
            │        └─ ">" → Data
            └─ string → TagName
                        ├─ SPACE → BeforeAttributeName
                        ├─ "/" → SelfClosingStartTag
                        └─ ">" → Data

    // loop in TagName
    TagName
    └─ SPACE → BeforeAttributeName
                └─ String / "=" → AttributeName
                                    └─ EOF → AfterAttributeName
                                                    ├─ "=" → BeforeAttributeValue
                                                    ├─ "/" → SelfClosingStartTag
                                                    └─ ">" → Data
    
    // Branching attribute values
    BeforeAttributeValue
    ├─ '"' → AttributeValueDoubleQuoted
    ├─ "'" → AttributeValueSingleQuoted
    └─ anything else → AttributeValueUnquoted

    (AttributeValue*)
    └─ """ → AfterAttributeValueQuoted
                ├─ SPACE → BeforeAttributeName
                ├─ "/" → SelfClosingStartTag
                └─ ">" → Data

    // Balancing self-closing tag
    SelfClosingStartTag
    └─ ">" → Data

    // A separate loop for scripts (exception handling)
    Data
    └─ "<script>" → ScriptData
                        └─ "<" → ScriptDataLessThanSign
                                └─ "/" → ScriptDataEndTagOpen
                                            └─ String → ScriptDataEndTagName
                                                        └─ Match confirmation → Data

    //
    ScriptDataEndTagName
    └─ Temporarily save characters for comparison → TemporaryBuffer
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    // https://html.spec.whatwg.org/multipage/parsing.html#data-state
    // Default state that emits text until a < starts a tag.
    // LESS-THAN SIGN (<) -> TagOpen
    Data,

    // https://html.spec.whatwg.org/multipage/parsing.html#tag-open-state
    // Entered after < to decide whether this is a start tag, end tag, or something else.
    // ASCII alpha -> TagName
    // SOLIDUS (/) -> EndTagOpen
    // Anything else -> Reconsume in the Data state.
    TagOpen,

    // https://html.spec.whatwg.org/multipage/parsing.html#end-tag-open-state
    // After </, determines whether a valid end tag name follows.
    // GREATRE-THAN SIGN (>) -> Data
    EndTagOpen,

    // https://html.spec.whatwg.org/multipage/parsing.html#tag-name-state
    // Reads and accumulates the tag’s name.
    // SPACE -> BeforeAttributeName
    // SOLIDUS (/) -> SelfClosingStartTag
    // GREATRE-THAN SIGN (>) -> Data
    // ASCII upper alpha -> Append the lowercase version of the current input character to the current tag token's tag name.
    // Anything else (normal key input) -> Append the current input character to the current tag token's tag name.
    TagName,

    // https://html.spec.whatwg.org/multipage/parsing.html#before-attribute-name-state
    // Skips whitespace before starting a new attribute name.
    // SPACE -> ignore the character
    // SOLIDUS (/) or EQUALS SIGN (=) or EOF -> AfterAttributeName
    // normal key input -> AttributeName (start new attribute)
    BeforeAttributeName,

    // https://html.spec.whatwg.org/multipage/parsing.html#attribute-name-state
    // Reads and accumulates an attribute’s name.
    // SPACE or SOLIDUS (/) or GREATER-THAN SIGN (>) or EOF -> AfterAttributeName
    // EQUALS SIGN (=) -> BeforeAttributeValue
    // ASCII upper alpha -> Append the lowercase version of the current input character to the current attribute's name.
    // Anything else -> Append the current input character to the current attribute's name.
    AttributeName,

    // https://html.spec.whatwg.org/multipage/parsing.html#after-attribute-name-state
    // Handles whitespace or = after finishing an attribute name.
    // SPACE -> Ignore
    // SOLIDUS (/) -> SelfClosingStartTag
    // EQUALS SIGN (=) -> BeforeAttributeValue
    // GREATER-THAN SIGN (>) -> Data, Emit the current token
    AfterAttributeName,

    // https://html.spec.whatwg.org/multipage/parsing.html#before-attribute-value-state
    // SPACE -> Ignore
    // QUOTATION MARK (") -> AttributeValueDoubleQuoted
    // APOSTROPHE (') -> AttributeValueSingleQuoted
    // anything else -> Reconsume in the attribute value (unquoted) state.
    BeforeAttributeValue,

    // https://html.spec.whatwg.org/multipage/parsing.html#attribute-value-(double-quoted)-state
    // QUOTATION MARK (") -> AfterAttributeValueQUoted
    // Anything else -> Append the current input character to the current attribute's value.
    AttributeValueDoubleQuoted,

    // https://html.spec.whatwg.org/multipage/parsing.html#attribute-value-(single-quoted)-state
    // APOSTROPHE (') -> AfterAttributeValueQuoted
    // Anything else -> Append the current input character to the current attribute's value.
    AttributeValueSingleQuoted,

    // https://html.spec.whatwg.org/multipage/parsing.html#attribute-value-(unquoted)-state
    AttributeValueUnquoted,

    // https://html.spec.whatwg.org/multipage/parsing.html#after-attribute-value-(quoted)-state
    AfterAttributeValueQuoted,

    // https://html.spec.whatwg.org/multipage/parsing.html#self-closing-start-tag-state
    SelfClosingStartTag,

    // https://html.spec.whatwg.org/multipage/parsing.html#script-data-state
    ScriptData,

    // https://html.spec.whatwg.org/multipage/parsing.html#script-data-less-than-sign-state
    // A state that determines whether the "/" in <script> is a closing tag or just a character.
    ScriptDataLessThanSign,

    // https://html.spec.whatwg.org/multipage/parsing.html#script-data-end-tag-open-state
    ScriptDataEndTagOpen,
    // https://html.spec.whatwg.org/multipage/parsing.html#script-data-end-tag-name-state
    ScriptDataEndTagName,

    // https://html.spec.whatwg.org/multipage/parsing.html#temporary-buffer
    TemporaryBuffer,
}

// HtmlTokenizer stores information for lexical analysis
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlTokenizer {
    // state of state machine
    // e.g. DataState, TagOpenState, etc.
    state: State,
    // current position in input
    pos: usize,
    reconsume: bool,
    latest_token: Option<HtmlToken>,
    // html input as vector of chars
    input: Vec<char>,
    buf: String,
}

impl HtmlTokenizer {
    // constructor
    pub fn new(html: String) -> Self {
        Self {
            state: State::Data,
            pos: 0,
            reconsume: false,
            latest_token: None,
            // store HTML string as vector of chars
            input: html.chars().collect(),
            buf: String::new(),
        }
    }

    fn is_eof(&self) -> bool {
        // return EoF token
        self.pos > self.input.len()
    }

    fn consume_next_input(&mut self) -> char {
        let c = self.input[self.pos];
        self.pos += 1;
        c
    }

    fn reconsume_input(&mut self) -> char {
        self.reconsume = false;
        self.input[self.pos - 1]
    }

    fn create_tag(&mut self, start_tag_token: bool) {
        // Create a StartTag or EndTag token and set it in the latest_token field
        if start_tag_token {
            self.latest_token = Some(HtmlToken::StartTag {
                tag: String::new(),
                self_closing: false,
                attributes: Vec::new(),
            });
        } else {
            self.latest_token = Some(HtmlToken::EndTag {
                tag: String::new()
            });
        }
    }

    fn append_tag_name(&mut self, c: char) {
        assert!(self.latest_token.is_some());

        if let Some(t) = self.latest_token.as_mut() {
            match t {
                HtmlToken::StartTag {
                    ref mut tag,
                    // "_" means this value is not used
                    self_closing: _,
                    attributes: _,
                } => tag.push(c),
                HtmlToken::EndTag {
                    ref mut tag
                } => tag.push(c),
                _ => panic!("`latest_token` should be either StartTag or EndTag"),
            }
        }
    }

    fn take_latest_token(&mut self) -> Option<HtmlToken> {
        assert!(self.latest_token.is_some());

        let t = self.latest_token.as_ref().cloned();

        // reset latest_token
        self.latest_token = None;
        assert!(self.latest_token.is_none());

        t
    }

    fn start_new_attribute(&mut self) {
        assert!(self.latest_token.is_some());

        if let Some(t) = self.latest_token.as_mut() {
            match t {
                HtmlToken::StartTag {
                    tag: _,
                    self_closing: _,
                    ref mut attributes,
                } => {
                    attributes.push(Attribute::new());
                }
                _ => panic!("`latest_token` should be either StartTag"),
            }
        }
    }

    fn append_attribute(&mut self, c: char, is_name: bool) {
        assert!(self.latest_token.is_some());

        if let Some(t) = self.latest_token.as_mut() {
            match t {
                    HtmlToken::StartTag {
                    tag: _,
                    self_closing: _,
                    ref mut attributes,
                } => {
                    let len = attributes.len();
                    // check precall start_new_attribute and make attributes instance
                    // If the attribute has not been created, an error occurs.
                    // assert!() If the result is false, an error is output.
                    assert!(len > 0);

                    // renderer/html/attribute.rsattribute.rs
                    // If is_name is true, add the attribute name. 
                    // // If it's false, add the attribute value.
                    attributes[len - 1].add_char(c, is_name);
                }
                _ => panic!("`latest_token` should be either StartTag"),
            }
        }
    }

    fn set_self_closing_flag(&mut self) {
        assert!(self.latest_token.is_some());

        if let Some(t) = self.latest_token.as_mut() {
            match t {
                HtmlToken::StartTag {
                    tag: _,
                    ref mut self_closing,
                    attributes: _,
                } => {
                    *self_closing = true;
                }
                _ => panic!("`latest_token` should be either StartTag"),
            }
        }
    }
}

impl Iterator for HtmlTokenizer {
    type Item = HtmlToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            return None;
        }

        loop {
            // "Reconsume" means that you only update the state and reuse the characters you used.
            let c = match self.reconsume {
                // Returns the character at the current position (pos) from the input string, 
                // and advances the position of pos by one.
                // Each time you call "consume_next_input()" you can consume a character.
                true => self.reconsume_input(),
                // Returns the character from the string just before the current position (pos - 1).
                false => self.consume_next_input(),
            };

            match self.state {
                State::Data => {
                    if c == '<' {
                        self.state = State::TagOpen;
                        continue;
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    return Some(HtmlToken::Char(c));
                }
                State::TagOpen => {
                    if c == '/' {
                        self.state = State::EndTagOpen;
                        continue;
                    }

                    if c.is_ascii_alphabetic() {
                        // reconsume a current character
                        self.reconsume = true;
                        self.state = State::TagName;
                        self.create_tag(true);
                        continue;
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    // Anything else -> reconsume in Data state
                    self.reconsume = true;
                    self.state = State::Data;
                }
                State::EndTagOpen => {
                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    if c.is_ascii_alphabetic() {
                        self.reconsume = true;
                        self.state = State::TagName;
                        self.create_tag(false);
                        continue;
                    }
                }
                State::TagName => {
                    // HTML Tag name grammer
                    // In HTML, the character immediately after < must be an English character (tag name).
                    // <p> is OK, < p> is error

                    // Therefore, this ' ' is determined to be after the tag name has already been entered.
                    if c == ' ' {
                        self.state = State::BeforeAttributeName;
                        continue;
                    }

                    if c == '/' {
                        self.state = State::SelfClosingStartTag;
                        continue;
                    }

                    if c == '>' {
                        self.state = State::Data;
                        // StartTag or EndTag
                        return self.take_latest_token();
                    }

                    if c.is_ascii_uppercase() {
                        self.append_tag_name(c.to_ascii_lowercase());
                        continue;
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    // Anything else -> Append the current input character to the current tag token's tag name.
                    self.append_tag_name(c);
                }

                State::BeforeAttributeName => {
                    // When tag is self-closing tag
                    // e.g. <br />, <img />
                    // c == '/' is true
                    if c == '/' || c == '>' || self.is_eof() {
                        self.reconsume = true;
                        self.state = State::AfterAttributeName;
                        continue;
                    }

                    // start new attribute
                    self.reconsume = true;
                    self.state = State::AttributeName;
                    self.start_new_attribute();
                }
                State::AttributeName => {
                    if c == ' ' || c == '/' || c == '>' || self.is_eof() {
                        self.reconsume = true;
                        self.state = State::AfterAttributeName;
                        continue;
                    }

                    if c == '=' {
                        self.state = State::BeforeAttributeValue;
                        continue;
                    }

                    if c.is_ascii_uppercase() {
                        self.append_attribute(c.to_ascii_lowercase(), /*is_name*/ true);
                        continue;
                    }

                    // Anything else, lower character etc.
                    self.append_attribute(c, /*is_name*/ true);
                }
                State::AfterAttributeName => {
                    if c == ' ' {
                        // ignore space
                        continue;
                    }

                    if c == '/' {
                        self.state = State::SelfClosingStartTag;
                        continue;
                    }

                    if c == '=' {
                        self.state = State::BeforeAttributeValue;
                        continue;
                    }

                    if c == '>' {
                        // Emit the current token
                        self.state = State::Data;
                        return self.take_latest_token();
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    // Anything else
                    // Start a new attribute in the current tag token. 
                    // Set that attribute name and value to the empty string. 
                    // Reconsume in the attribute name state.
                    self.reconsume = true;
                    self.state = State::AttributeName;
                    self.start_new_attribute();
                }
                State::BeforeAttributeValue => {
                    if c == ' ' {
                        // Ignore Space
                        continue;
                    }

                    if c == '"' {
                        self.state = State::AttributeValueDoubleQuoted;
                        continue;
                    }

                    if c == '\'' {
                        self.state = State::AttributeValueSingleQuoted;
                        continue;
                    }

                    // anything else -> Reconsume in the attribute value (unquoted) state.
                    self.reconsume = true;
                    self.state = State::AttributeValueUnquoted;
                }
                State::AttributeValueDoubleQuoted => {
                    if c == '"' {
                        self.state = State::AfterAttributeValueQuoted;
                        continue;
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    // Anything else -> Append the current input character to the current attribute's value.
                    self.append_attribute(c, /*is_name*/ false);
                }
                State::AttributeValueSingleQuoted => {
                    if c == '\'' {
                        self.state = State::AfterAttributeValueQuoted;
                        continue;
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    self.append_attribute(c, /*is_name*/ false);
                }
                State::AttributeValueUnquoted => {
                    if c == ' ' {
                        self.state = State::BeforeAttributeName;
                        continue;
                    }

                    if c == '>' {
                        self.state = State::Data;
                        return self.take_latest_token();
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    self.append_attribute(c, /*is_name*/ false);
                }
                State::AfterAttributeValueQuoted => {
                    if c == ' ' {
                        self.state = State::BeforeAttributeName;
                        continue;
                    }

                    if c == '/' {
                        self.state = State::SelfClosingStartTag;
                        continue;
                    }

                    if c == '>' {
                        // Emit the current tag token
                        self.state = State::Data;
                        return self.take_latest_token();
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    self.reconsume = true;
                    self.state = State::BeforeAttributeValue;
                }

                State::SelfClosingStartTag => {
                    if c == '>' {
                        self.set_self_closing_flag();
                        self.state = State::Data;
                        return self.take_latest_token();
                    }

                    if self.is_eof() {
                        // invalid parse error.
                        return Some(HtmlToken::Eof);
                    }
                }
                
                State::ScriptData => {
                    if c == '<' {
                        self.state = State::ScriptDataLessThanSign;
                        continue;
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    return Some(HtmlToken::Char(c));
                }
                State::ScriptDataLessThanSign => {
                    // A state that determines whether the "/" in <script> is 
                    // a closing tag or just a character.
                    if c == '/' {
                        self.buf = String::new();
                        self.state = State::ScriptDataEndTagOpen;
                        continue;
                    }

                    // Anything else -> ScriptData
                    self.reconsume = true;
                    self.state = State::ScriptData;
                    return Some(HtmlToken::Char('<'));
                }
                State::ScriptDataEndTagOpen => {
                    if c.is_ascii_alphabetic() {
                        self.reconsume = true;
                        self.state = State::ScriptDataEndTagName;
                        self.create_tag(false);
                        continue;
                    }

                    //In the documentation, it returns two character tokens, "<" and "/".
                    // However, in our code, next() can only return one token.
                    // Therefore, it only returns the "<" token.
                    self.reconsume = true;
                    self.state = State::ScriptData;
                    return Some(HtmlToken::Char('<'));
                }
                State::ScriptDataEndTagName => {
                    if c == '>' {
                        self.state = State::Data;
                        return self.take_latest_token();
                    }

                    if c.is_ascii_alphabetic() {
                        self. buf.push(c);
                        self.append_tag_name(c.to_ascii_lowercase());
                        continue;
                    }

                    // e.g.
                    // </scripx
                    // we should return "</scripx" as string
                    self.state = State::TemporaryBuffer;
                    self.buf = String::from("</") + &self.buf;
                    self.buf.push(c);
                    continue;
                }
                State::TemporaryBuffer => {
                    self.reconsume = true;

                    if self.buf.chars().count() == 0 {
                        self.state = State::ScriptData;
                        continue;
                    }

                    // delete first character
                    let c = self
                        .buf
                        .chars()
                        .nth(0)
                        .expect("self.buf should have at least 1 char");

                    self.buf.remove(0);
                    return Some(HtmlToken::Char(c));
                }

            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alloc::string::ToString;
    use alloc::vec;

    #[test]
    fn test_empty() {
        let html = "".to_string();
        let mut tokenizer = HtmlTokenizer::new(html);
        assert!(tokenizer.next().is_none());
    }

    #[test]
    fn test_start_and_end_tag() {
        let html = "<body></body>".to_string();
        let mut tokenizer = HtmlTokenizer::new(html);
        let expected = [
            HtmlToken::StartTag {
                tag: "body".to_string(),
                self_closing: false,
                attributes: Vec::new(),
            },
            HtmlToken::EndTag {
                tag: "body".to_string(),
            },
        ];
        for e in expected {
            assert_eq!(Some(e), tokenizer.next());
        }
    }

    #[test]
    fn test_attributes() {
        let html = "<p class=\"A\" id='B' foo=bar></p>".to_string();
        let mut tokenizer = HtmlTokenizer::new(html);
        let mut attr1 = Attribute::new();

        attr1.add_char('c', true);
        attr1.add_char('l', true);
        attr1.add_char('a', true);
        attr1.add_char('s', true);
        attr1.add_char('s', true);
        // value
        attr1.add_char('A', false);

        let mut attr2 = Attribute::new();
        attr2.add_char('i', true);
        attr2.add_char('d', true);
        attr2.add_char('B', false);

        let mut attr3 = Attribute::new();
        attr3.add_char('f', true);
        attr3.add_char('o', true);
        attr3.add_char('o', true);
        // value
        attr3.add_char('b', false);
        attr3.add_char('a', false);
        attr3.add_char('r', false);

        let expected = [
            HtmlToken::StartTag {
                tag: "p".to_string(),
                self_closing: false,
                attributes: vec![attr1, attr2, attr3],
            },
            HtmlToken::EndTag {
                tag: "p".to_string(),
            },
        ];
        
        for e in expected {
            assert_eq!(Some(e), tokenizer.next());
        }
    }

    #[test]
    fn test_self_closing_tag() {
        let html = "<img />".to_string();
        let mut tokenizer = HtmlTokenizer::new(html);
        let expected = [HtmlToken::StartTag {
            tag: "img".to_string(),
            self_closing: true,
            attributes: Vec::new(),
        }];

        for e in expected {
            assert_eq!(Some(e), tokenizer.next());
        }
    }

    #[test]
    fn test_script_tag() {
        let html = "<script>js code;</script>".to_string();
        let mut tokenizer = HtmlTokenizer::new(html);
        let expected = [
            HtmlToken::StartTag {
                tag: "script".to_string(),
                self_closing: false,
                attributes: Vec::new(),
            },
            
            HtmlToken::Char('j'),
            HtmlToken::Char('s'),
            HtmlToken::Char(' '),
            HtmlToken::Char('c'),
            HtmlToken::Char('o'),
            HtmlToken::Char('d'),
            HtmlToken::Char('e'),
            HtmlToken::Char(';'),

            HtmlToken::EndTag {
                tag: "script".to_string(),
            },
        ];

        for e in expected {
            assert_eq!(Some(e), tokenizer.next());
        }
    }
}