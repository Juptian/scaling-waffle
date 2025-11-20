use std::collections::LinkedList;
use std::fs::File;
use std::io::Read;
use regex::Regex;
use crate::syntax_token::*;

pub mod syntax_token;
pub struct Lexer {
    file: Vec<char>,
    tokens: LinkedList<LexerToken>,
    idx: usize,
    current_location: Location,
    current_line: usize,
    current_column: usize,
}

pub struct LexerToken {
    kind: SyntaxKind,
    data: String,
    location: Location,
    length: usize,
}

impl Lexer {
    pub fn new(file_path: &str) -> Self {
        let mut vec = Vec::new();
        if  File::open(file_path).unwrap().read_to_end(&mut vec).is_ok() {
            let mut final_vec = Vec::new();
            for i in vec {
                final_vec.push(char::from(i))
            }

            Lexer {
                file: final_vec,
                tokens: LinkedList::new(),
                idx: 0,
                current_location: Location::new(0, 0),
                current_line: 0,
                current_column: 0,
            }

        } else {
            let str: &str = file_path;
            panic!("Cannot read from file: {}", str);
        }

    }

    pub fn lex(&mut self) {
        while self.idx < self.file.len() {
            let cur = self.current();
            // If we're at a newline char, we can update current line
            self.current_location.set_line_and_column(self.current_line, self.current_column);
            self.parseChar(cur);
        }
    }

    fn peek(&self, offset: usize) -> char {
        if let Some(c) = self.file.get(self.idx + offset).cloned()  {
            c
        } else {
            char::default()
        }
    }

    fn expect(&self, expected: SyntaxKind) -> bool {
        let char = char::from(self.peek(self.idx));
        if let Some(actual) = SyntaxKind::from_char(char) {
            actual == expected
        } else {
            false
        }
    }

    fn current(&self) -> char {
        self.peek(0)
    }

    fn parseChar(&mut self, c: char) -> Option<LexerToken> {
        let data = c.to_string();
        match c {
            '\n' => {
                self.current_line += 1;
                self.current_column = 0;
                Some(LexerToken {
                    kind: SyntaxKind::NEW_LINE,
                    data,
                    location: self.current_location.clone(),
                    length: 1,
                })
            }
            '+' => {
                self.current_column += 1;
                Some(LexerToken {
                    kind: SyntaxKind::PLUS,
                    data,
                    location: self.current_location.clone(),
                    length: 1,
                })
            }


            _ => {
                // Could be a string literal or a number literal
                let numberRegex = Regex::new("[0-9]").unwrap();
                let strs = Regex::new("[a-zA-Z]").unwrap();
                if let Some(nums) = numberRegex.captures(c.to_string().as_str()) {
                    // We have numbers
                    Some(self.readNumbers(&numberRegex))
                } else if let Some(str) = strs.captures(c.to_string().as_str()) {
                    // We probably have characters
                    Some(self.readChars(&strs))
                } else {
                    // We have either an unexpected token or a whitespace token
                    if self.expect(SyntaxKind::WHITESPACE) {
                        None
                    } else {
                        panic!("Unable to parse token!")
                    }
                }

            }
        }
    }

    fn readNumbers(&mut self, numRegex: &Regex) -> LexerToken {
        let start = self.idx;
        while self.idx < self.file.len() &&
              numRegex.captures(self.file[self.idx].to_string().as_str()).is_some()
            {
                self.idx += 1;
        }

        let slice = Vec::from(&self.file[start..self.idx]);
        let st = vec_to_str(slice);
        let len = st.len();
        let result = LexerToken {
            kind: SyntaxKind::NUMBER_LITERAL,
            data: st,
            location: self.current_location.clone(),
            length: len,
        };
        self.current_column += len;
        result
    }

    fn readChars(&self, strRegex: &Regex) -> LexerToken {
        todo!()
    }
}

fn vec_to_str(v: Vec<char>) -> String {
    let mut result = Vec::new();
    for i in v {
        result.push(char::from(i).to_string());
    }
    result.join("")
}

#[cfg(test)]
mod tests {
    use super::Lexer;

}