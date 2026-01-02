use crate::renderer::css::token::CssTokenizer;
use core::iter::Peekable;
use alloc::vec::Vec;
use crate::alloc::string::ToString;
use alloc::string::String;
use crate::renderer::css::token::CssToken;

#[derive(Debug, Clone)]
pub struct CssParser {
    t: Peekable<CssTokenizer>,
}

impl CssParser {
    pub fn new(t: CssTokenizer) -> Self {
        Self { t: t.peekable() }
    }

    fn consume_component_value(&mut self) -> ComponentValue {
        self.t
            .next()
            .expect("should have a token in consume_component_value")
    }

    fn consume_ident(&mut self) -> String {
        let token = match self.t.next() {
            Some(t) => t,
            None => panic!("should have be a token but got None"),
        };

        match token {
            CssToken::Ident(ref ident) => ident.to_string(),
            _ => {
                panic!("Parse error: {:?} is an unexpected token.", token);
            }
        }
    }

    fn consume_declaration(&mut self) -> Option<Declaration>{
        if self.t.peek().is_some() {
            return None;
        }

        let mut declaration = Declaration::new();
        // Set an identifier for a declaration structure property
        declaration.set_property(self.consume_ident());

        match self.t.next() {
            Some(token) => match token {
                CssToken::Colon => {}
                _ => return None,
            },
            None => return None,
        }

        declaration.set_value(self.consume_component_value());
        Some(declaration)
    }

    fn consume_list_of_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = Vec::new();

        loop {
            let token = match self.t.peek() {
                Some(t) => t,
                None => return declarations,
            };

            match token {
                CssToken::CloseCurly => {
                    assert_eq!(self.t.next(), Some(CssToken::CloseCurly));
                    return declarations;
                }
                CssToken::SemiColon => {
                    assert_eq!(self.t.next(), Some(CssToken::SemiColon));
                    // One declaration is completed, do nothing
                }
                CssToken::Ident(ref _ident) => {
                    if let Some(declaration) = self.consume_declaration() {
                        declarations.push(declaration);
                    }
                }
                _ => {
                    self.t.next();
                }
            }
        }
    }

    fn consume_selector(&mut self) -> Selector {
        let token = match self.t.next() {
            Some(t) => t,
            None => panic!("should have a token but got None"),
        };

        match token {
            CssToken::HashToken(value) => Selector::IdSelector(value[1..].to_string()),
            CssToken::Delim(delim) => {
                if delim == '.' {
                    return Selector::ClassSelector(self.consume_ident());
                }
                panic!("Parse error: {:?} is an unexpected token.", token);
            }
            CssToken::Ident(ident) => {
                // Selectors like a:hover are treated as type selectors, 
                // so when a ':' appears, the token advances to just after the start of the declaration block.
                if self.t.peek() == Some(&CssToken::Colon) {
                    while self.t.peek() != Some(&CssToken::OpenCurly) {
                        self.t.next();
                    }
                }
                Selector::TypeSelector(ident.to_string())
            }
            CssToken::AtKeyword(_keyword) => {
                // To ignore the @ rule, 
                // advance the token to just before the start of the declaration block.
                while self.t.peek() != Some(&CssToken::OpenCurly) {
                    self.t.next();
                }
                Selector::UnknownSelector
            }
            _ => {
                self.t.next();
                Selector::UnknownSelector
            }
        }
    }

    fn consume_qualified_rule(&mut self) -> Option<QualifiedRule> {
        let mut rule = QualifiedRule::new();

        loop {
            let token = match self.t.peek() {
                Some(t) => t,
                None => return None,
            };

            match token {
                CssToken::OpenCurly => {
                    assert_eq!(self.t.next(), Some(CssToken::OpenCurly));
                    rule.set_declarations(self.consume_list_of_declarations());
                    return Some(rule);
                }
                _ => {
                    rule.set_selector(self.consume_selector());
                }
            }
        }
    }

    fn consume_list_of_rules(&mut self) -> Vec<QualifiedRule> {
        let mut rules = Vec::new();

        loop {
            let token = match self.t.peek() {
                Some(t) => t,
                None => return rules,
            };

            match token {
                // When the @ token appears, 
                // it indicates the beginning of a rule such as @import, 
                // which imports other CSS, or @media, which represents a media query.
                CssToken::AtKeyword(_kerword) => {
                    // This time, rules starting with @ are not supported, so ignore them.
                    let _rule = self.consume_qualified_rule();
                }
                _ => {
                    // Interpreting one rule
                    let rule = self.consume_qualified_rule();
                    match rule {
                        Some(r) => rules.push(r),
                        None => return rules,
                    }
                }
            }
        }
    }

    pub fn parse_stylesheet(&mut self) -> StyleSheet {
        let mut sheet = StyleSheet::new();
        // Create a list of rules from tokens  and set it in a field
        sheet.set_rules(self.consume_list_of_rules());
        sheet
    }
}

// Root node
// StyleSheet has multiple rules
// StyleSheet
//  - rule1
//  - rule2
//  ...
#[derive(Debug, Clone, PartialEq)]
pub struct StyleSheet {
    pub rules: Vec<QualifiedRule>,
}

impl StyleSheet {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn set_rules(&mut self, rules: Vec<QualifiedRule>) {
        self.rules = rules;
    }
}

// one rule is expressed with this structure
#[derive(Debug, Clone, PartialEq)]
pub struct QualifiedRule {
    pub selector: Selector,
    pub declarations: Vec<Declaration>,
}

impl QualifiedRule {
    pub fn new() -> Self {
        Self {
            selector: Selector::TypeSelector("".to_string()),
            declarations: Vec::new(),
        }
    }

    pub fn set_selector(&mut self, selector: Selector) {
        self.selector = selector;
    }

    pub fn set_declarations(&mut self, declarations: Vec<Declaration>) {
        self.declarations = declarations;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selector {
    TypeSelector(String),
    ClassSelector(String),
    IdSelector(String),
    UnknownSelector,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub property: String,
    pub value: ComponentValue,
}

impl Declaration {
    // constructor
    pub fn new() -> Self {
        Self {
            property: String::new(),
            value: ComponentValue::Ident(String::new()),
        }
    }

    pub fn set_property(&mut self, property: String) {
        self.property = property;
    }

    pub fn set_value(&mut self, value: ComponentValue) {
        self.value = value;
    }
}

pub type ComponentValue = CssToken;