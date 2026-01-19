use crate::lexer::syntax_token::SyntaxKind;
use crate::lexer::LexerToken;
use dyn_clone::{clone_box, DynClone};
use std::collections::LinkedList;


pub trait Statement: Expression + DynClone {
    fn get_expressions(&self) -> Option<Vec<Box<dyn Expression>>> ;
}


pub trait Expression: DynClone {

}
pub struct Parser {
    current: Option<LexerToken>,
    lexer_tokens: LinkedList<LexerToken>,
    parsed_tokens: LinkedList<Box<dyn Statement>>
}

// Statement
pub struct FunctionStatement {
    func_type: LexerToken,
    identifier: LexerToken,
    open_paren: LexerToken,
    pub parameters: Vec<Box<LexerToken>>,
    close_paren: LexerToken,
    func_body: Body,
}

pub struct Body {
    open_curly: LexerToken,
    pub statements: Vec<Box<dyn Statement>>,
    close_curly: LexerToken,
}

// Expressions
#[derive(Clone)]
pub struct ReturnExpr {
    return_kw: LexerToken,
    pub value: LexerToken,
    semicolon: LexerToken,
}
impl Parser {
    pub fn new(lexed_tokens: LinkedList<LexerToken>) -> Self {
        Parser {
            current: None,
            lexer_tokens: lexed_tokens,
            parsed_tokens: LinkedList::new(),
        }
    }

    pub fn expect(&mut self, expected: SyntaxKind) -> bool{
        if self.current.clone().unwrap().kind != expected {
            return false
        }
        self.current = self.lexer_tokens.pop_front();
        true
    }

    fn eat_current_token(&mut self) {
        self.current = self.lexer_tokens.pop_front();
    }

    // We're expecting
    /*
    int main(void) {
        return <const>;
    }
    */
    pub fn start_parsing(&mut self) {
        self.current = self.lexer_tokens.pop_front();
        while self.current.is_some() {
            self.parse_statement();
        }
    }

    fn parse_statement(& mut self) -> Box<dyn Statement> {
        let cur = self.current.as_ref().unwrap();
        // If we have a type keyword we try to parse a function, then an assignment
        if SyntaxKind::is_type_keyword(&cur.kind) {

            // I probably shouldn't be doing this however
            // It makes sense to keep it as the same reference
            // We don't really want it to be duped.
            // Sure it limits ownership but that's kinda the whole point of rust
            let func = Box::new(self.parse_func());
            let pushback = (*func).clone();

            self.parsed_tokens.push_back(func);
            Box::new(pushback)
        } else if cur.kind == SyntaxKind::ReturnKeyword {
            Box::new(self.parse_return())
        } else {
            // Error
            panic!()
        }
    }
    fn parse_func(&mut self) -> FunctionStatement {
        // We know we already have a type keyword
        let func_type = self.current.as_ref().unwrap().clone();
        self.eat_current_token();
        let identifier = self.current.as_ref().unwrap().clone();
        self.expect(SyntaxKind::StringLiteral);

        let open_paren = self.current.as_ref().unwrap().clone();
        if !self.expect(SyntaxKind::OpenParen) {
            // Womp womp
        }
        let mut parameters: Vec<Box<LexerToken>> = Vec::new();
        while self.current.as_ref().unwrap().kind != SyntaxKind::CloseCurly {
            // Right now we only support the word void
            let param = Box::new(self.current.as_ref().unwrap().clone());
            self.expect(SyntaxKind::VoidKeyword);
            parameters.push(param);
        }

        let close_paren = self.current.as_ref().unwrap().clone();
        self.expect(SyntaxKind::CloseParen);
        let body = self.parse_body();
        FunctionStatement {
            func_type,
            identifier,
            open_paren,
            parameters,
            close_paren,
            func_body: body,
        }
    }

    fn parse_body(&mut self) -> Body {
        let open_curly = self.current.as_ref().unwrap().clone();
        self.expect(SyntaxKind::OpenCurly);

        // We need to parse body statements
        // Until we hit a closing curly bracket we don't care what's going on
        // It's great
        let mut statements: Vec<Box<dyn Statement>> = Vec::new();
        while self.current.as_ref().unwrap().kind != SyntaxKind::CloseCurly {
            let statement = self.parse_statement();
            statements.push(statement)
        }

        let close_curly = self.current.as_ref().unwrap().clone();
        self.expect(SyntaxKind::CloseCurly);

        Body {
            open_curly,
            statements,
            close_curly,
        }

    }
    fn parse_return(&mut self) -> ReturnExpr {
        // We're only supporting values right now
        // Yippie!
        let return_kw = self.current.as_ref().unwrap().clone();
        self.expect(SyntaxKind::ReturnKeyword);

        let value = self.current.as_ref().unwrap().clone();
        self.eat_current_token();

        let semicolon = self.current.as_ref().unwrap().clone();
        self.expect(SyntaxKind::Semicolon);

        ReturnExpr {
            return_kw,
            value,
            semicolon,
        }
    }


}

impl Body {
    pub fn new(open_curl: LexerToken, incoming_statements: Vec<Box<dyn Statement>>, close_curl: LexerToken) -> Self {
        Body {
            open_curly: open_curl,
            statements: incoming_statements,
            close_curly: close_curl,
        }
    }
}

impl Expression for Body {}

impl Statement for Body {
    fn get_expressions(&self) -> Option<Vec<Box<dyn Expression>>>
    {
        None
    }
}
impl Clone for Body {
    fn clone(&self) -> Self {
        let mut new_statements: Vec<Box<dyn Statement>>= Vec::new();
        for item in self.statements.iter().clone() {
            let item_as_ref = item.as_ref();
            let cloned_box = clone_box(item_as_ref);
            new_statements.push(cloned_box)

        }
        Body {
            open_curly: self.open_curly.clone(),
            statements: new_statements,
            close_curly: self.close_curly.clone(),
        }
    }
}

impl Expression for FunctionStatement {}

impl Statement for FunctionStatement {
    fn get_expressions(&self) -> Option<Vec<Box<dyn Expression>>>
    {
        None
    }

}


impl Clone for FunctionStatement {
    fn clone(&self) -> Self {
        FunctionStatement {
            func_type: self.func_type.clone(),
            identifier: self.identifier.clone(),
            open_paren: self.open_paren.clone(),
            parameters: self.parameters.clone(),
            close_paren: self.close_paren.clone(),
            func_body: self.func_body.clone(),
        }
    }
}

// Usually using a polymorphic approach you'd make Expressions derive from statements
// This should work?
// Realistically the difference between a statement
// and an expression are mostly semantic
// They're used differently, but we only care about
impl Statement for dyn Expression {
    fn get_expressions(&self) -> Option<Vec<Box<dyn Expression>>>
    {
        // Should this return Option(Vec::new(Box::new(self)))?
        None
    }
}

impl Expression for ReturnExpr {

}
impl Statement for ReturnExpr {
    fn get_expressions(&self) -> Option<Vec<Box<dyn Expression>>> {
        None
    }
}