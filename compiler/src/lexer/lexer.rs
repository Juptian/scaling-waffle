use std::collections::LinkedList;
use std::fmt;
use regex::Regex;
use crate::lexer::syntax_token::*;

pub mod syntax_token;

pub struct Lexer {
    pub file: Vec<char>,
    pub tokens: LinkedList<LexerToken>,
    idx: usize,
    current_location: Location,
    current_line: usize,
    current_column: usize,
}

pub struct LexerToken {
    pub kind: SyntaxKind,
    pub data: String,
    pub location: Location,
    pub length: usize,
}

impl Lexer {
    pub fn new(file_data: &str) -> Self {
        let vec = file_data.chars().collect::<Vec<char>>();
        if  !vec.is_empty() {
            let mut final_vec = Vec::new();
            for i in vec {
                final_vec.push(char::from(i))
            }

            Lexer {
                file: final_vec,
                tokens: LinkedList::new(),
                idx: 0,
                current_location: Location::new(0, 0),
                current_line: 1,
                current_column: 1,
            }
        } else {
            panic!("Cannot read from file: {0}", file_data);
        }

    }

    pub fn lex(&mut self) {
        while self.idx < self.file.len() {
            let cur = self.current();
            // If we're at a newline char, we can update current line
            self.current_location.set_line_and_column(self.current_line, self.current_column);
            let token = self.parse_char(cur);
            if let Some(token) = token {
                self.tokens.push_back(token);
            }
        }
    }

    fn peek(&self, offset: usize) -> char {
        if let Some(c) = self.file.get(self.idx + offset).cloned()  {
            c
        } else {
            char::default()  // \x00
        }
    }

    fn expect(&mut self, expected: SyntaxKind) -> bool {
        let char = char::from(self.peek(0));
        if let Some(actual) = SyntaxKind::from_char(char) {
            self.idx += 1;
            actual == expected
        } else {
            false
        }
    }

    fn current(&self) -> char {
        self.peek(0)
    }

    fn parse_char(&mut self, c: char) -> Option<LexerToken> {
        let data = c.to_string();

        let symbol = SyntaxKind::from_char(c);
        if let Some(mut kind) = symbol {
            // Once we know that it's a single or max two digit character
            // We can safely increase the index & column
            // Otherwise, our strings & numbers will miss the first character, and that's not good!
            self.idx += 1;
            self.current_column += 1;

            self.current_location.set_line_and_column(self.current_line, self.current_column - 1);

            match kind {
                // Multiline chars that will match at one char
                SyntaxKind::Equal => {
                    if self.current() == '=' {
                        kind = SyntaxKind::EqualEqual;
                    }
                }
                SyntaxKind::NewLine => {
                    self.current_line += 1;
                    self.current_column = 1;
                }
                _ => {}
            }

            let result = LexerToken {
                kind,
                data,
                location: self.current_location.clone(),
                length: 1
            };

            Some(result)

        } else {
            // Could be a string literal or a number literal
            let number_regex = Regex::new("[0-9]").unwrap();
            let strs = Regex::new("[a-zA-Z]").unwrap();

            if number_regex.captures(c.to_string().as_str()).is_some() {
                // We have numbers
                Some(self.read_numbers(&number_regex))

            } else if strs.captures(c.to_string().as_str()).is_some() {
                // We probably have characters
                Some(self.read_chars(&strs))
            } else if c.is_whitespace() {
                // As we're dealing with whitespace, we can safely just skip it.
                self.idx += 1;
                self.current_column += 1;
                None
            } else {
                //
                // We should never hit this section unless something went wrong
                // Everything should either
                // *    match on one character and turn into a token
                // *    Match on one character, and read ahead to turn into a different token
                // *    Neither of the above
                //      * Then they should be some form of string
                //      * Or number
                // If we ever end up here, we've exhausted our options.
                //
                panic!("Unable to lex character {0}", c)
            }
        }
    }

    fn read_numbers(&mut self, num_regex: &Regex) -> LexerToken {

        let st = self.read_while_match(num_regex);
        let len = st.len();
        let result = LexerToken {
            kind: SyntaxKind::NumberLiteral,
            data: st,
            location: self.current_location.clone(),
            length: len,
        };
        self.current_column += len;
        result
    }

    fn read_chars(&mut self, str_regex: &Regex) -> LexerToken {
        let st = self.read_while_match(str_regex);
        let len = st.len();

        let mut kind = SyntaxKind::StringLiteral;

        if let Some(new_kind) = SyntaxKind::from_str(st.as_str()) {
            kind = new_kind;
        }

        let result = LexerToken {
            kind,
            data: st,
            location: self.current_location.clone(),
            length: len
        };

        self.current_column += len;
        result
    }

    fn read_while_match(&mut self, reg: &Regex) -> String {
        let start_index = self.idx;
        while self.idx < self.file.len() &&  reg.captures(self.file[self.idx].to_string().as_str()).is_some() {
            self.idx += 1;
        }
        let slice = Vec::from(&self.file[start_index..self.idx]);
        self.vec_to_str(slice)
    }

    fn vec_to_str(&self, v: Vec<char>) -> String {
        let mut result = Vec::new();
        for i in v {
            result.push(char::from(i).to_string());
        }
        result.join("")
    }

}

impl fmt::Display for LexerToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = f.write_str("Token{ Data : ");
        let _ = f.write_str(self.data.as_str());
        let _ = f.write_str(", Location : ");
        let _ = f.write_str(self.location.to_string().as_str());
        Ok(())
    }
}

