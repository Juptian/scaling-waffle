use std::fmt;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub enum SyntaxKind {

    // KEYWORDS
    IntKeyword,
    CharKeyword,
    ReturnKeyword,

    // LITERALS
    NumberLiteral,
    StringLiteral,

    // SYMBOLS
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    Semicolon,
    Comma,
    Underscore,

    // MATH SYMBOLS
    Equal,
    Minus,
    Plus,
    EqualEqual,
    Star,
    Slash,

    // WHITESPACE
    NewLine,

    // We're just skipping over whitespace
}

impl SyntaxKind {
    pub fn from_char(c: char) -> Option<SyntaxKind> {
        use SyntaxKind::*;
        match c {
            // Symbols
            '(' => Some(OpenParen),
            ')' => Some(CloseParen),
            '{' => Some(OpenCurly),
            '}' => Some(CloseCurly),
            ';' => Some(Semicolon),
            ',' => Some(Comma),
            '_' => Some(Underscore),

            // Math symbols
            '=' => Some(Equal),
            '-' => Some(Minus),
            '+' => Some(Plus),
            '*' => Some(Star),
            '/' => Some(Slash),


            // Whitespace
            '\n' => Some(NewLine),

            // Default
            _ => {
                None
            }
        }
    }
    pub fn from_str(s: &str) -> Option<SyntaxKind> {
        use SyntaxKind::*;
        match s {
            "int" => Some(IntKeyword),
            "==" => Some(EqualEqual),
            "return" => Some(ReturnKeyword),
            "char" => Some(CharKeyword),
            _ => None
        }
    }
}


pub struct Location {
    pub(crate) line: usize,
    pub(crate) column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Location {
        Location {
            line,
            column,
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }
    pub fn column(&self) -> usize {
        self.column
    }

    pub fn line_and_column(&self) -> (usize, usize) {
        (self.line, self.column)
    }

    pub fn set_line(& mut self, l: usize) {
        self.line = l;
    }
    pub fn set_column(& mut self, c: usize) {
        self.column = c;
    }

    pub fn set_line_and_column(& mut self, l: usize, c: usize) {
        self.line = l;
        self.column = c;
    }
}

impl Clone for Location {
    fn clone(&self) -> Self {
        Location {
            line: self.line,
            column: self.column,
        }
    }
}
impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.line, self.column)
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Location) -> bool {
        self.line == other.line && self.column == other.column
    }
}

impl Eq for Location {}

impl Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.line, self.column)
    }
}
