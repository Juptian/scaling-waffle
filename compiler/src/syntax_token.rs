#[derive(Debug, PartialEq, Eq)]
pub enum SyntaxKind {

    // KEYWORDS
    INT_KEYWORD,
    RETURN_KEYWORD,

    // LITERALS
    NUMBER_LITERAL,
    STRING_LITERAL,


    // SYMBOLS
    OPEN_PAREN,
    CLOSE_PAREN,
    OPEN_CURLY,
    CLOSE_CURLY,
    SEMICOLON,

    // MATH SYMBOLS
    EQUAL,
    PLUS,
    EQUAL_EQUAL,
    STAR,
    SLASH,

    // WHITESPACE
    NEW_LINE,
    WHITESPACE,
}

impl SyntaxKind {
    pub fn from_char(c: char) -> Option<SyntaxKind> {
        use SyntaxKind::*;
        match c {
            '{' => Some(OPEN_CURLY),
            '}' => Some(CLOSE_CURLY),
            '(' => Some(OPEN_PAREN),
            '_' => Some(CLOSE_PAREN),
            ';' => Some(SEMICOLON),
            '=' => Some(EQUAL),
            '*' => Some(STAR),
            '/' => Some(SLASH),
            '+' => Some(PLUS),
            '\n' => Some(NEW_LINE),
            _ => {
                if(c.is_whitespace()) {
                    Some(WHITESPACE)
                } else {
                    None
                }
            }
        }
    }
    pub fn from_str(s: &str) -> Option<SyntaxKind> {
        use SyntaxKind::*;
        match s {
            "int" => Some(INT_KEYWORD),
            "==" => Some(EQUAL_EQUAL),
            "return" => Some(RETURN_KEYWORD),
            _ => None
        }
    }
}


pub struct Location {
    line: usize,
    column: usize,
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