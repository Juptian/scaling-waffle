#[cfg(test)]
mod lexer_tests {
    use crate::lexer::Lexer;
    use crate::lexer::syntax_token::*;

    #[test]
    fn lexing_whitespace() {

        let str = "\n";
        let mut lexer = Lexer::new(str);

        lexer.lex();
        assert_eq!(lexer.file.len(), str.len());
        assert_eq!(lexer.tokens.len(), 1);
        assert_eq!(lexer.tokens.front().unwrap().kind, SyntaxKind::NewLine);
        let token = lexer.tokens.pop_front().unwrap();
        assert_eq!(token.data, str);

        let str = "    ";
        let mut lexer = Lexer::new(str);
        lexer.lex();
        assert_eq!(lexer.file.len(), str.len());
        assert_eq!(lexer.tokens.len(), 0);
    }

    #[test]
    fn lexing_basic_strings() {
        let str = "foobar";
        let mut lexer = Lexer::new(str);
        lexer.lex();

        assert_eq!(lexer.tokens.front().unwrap().kind, SyntaxKind::StringLiteral);
        assert_eq!(lexer.file.len(), str.len());
        assert_eq!(lexer.tokens.len(), 1);
        let token = lexer.tokens.pop_front().unwrap();
        assert_eq!(token.data, str);


        let str = "foo bar";
        let mut lexer = Lexer::new(str);
        lexer.lex();

        assert_eq!(lexer.tokens.front().unwrap().kind, SyntaxKind::StringLiteral);
        assert_eq!(lexer.file.len(), str.len());
        assert_eq!(lexer.tokens.len(), 2);
        lexer.tokens.pop_front();
        assert_eq!(lexer.tokens.front().unwrap().kind, SyntaxKind::StringLiteral);
    }

    #[test]
    fn lexing_keywords() {

        let str = "int";
        let mut lexer = Lexer::new(str);
        lexer.lex();

        assert_eq!(lexer.tokens.front().unwrap().kind, SyntaxKind::IntKeyword);
        assert_eq!(lexer.file.len(), str.len());
        assert_eq!(lexer.tokens.len(), 1);
        let token = lexer.tokens.pop_front().unwrap();
        assert_eq!(token.location, Location {
            column: 1,
            line: 1,
        });



        let str = "return";
        let mut lexer = Lexer::new(str);
        lexer.lex();

        assert_eq!(lexer.tokens.front().unwrap().kind, SyntaxKind::ReturnKeyword);
        assert_eq!(lexer.file.len(), str.len());
        assert_eq!(lexer.tokens.len(), 1);

        let str = "char";
        let mut lexer = Lexer::new(str);
        lexer.lex();

        assert_eq!(lexer.tokens.front().unwrap().kind, SyntaxKind::CharKeyword);
        assert_eq!(lexer.file.len(), str.len());
        assert_eq!(lexer.tokens.len(), 1);


        let str = "void";
        let mut lexer = Lexer::new(str);
        lexer.lex();

        assert_eq!(lexer.tokens.front().unwrap().kind, SyntaxKind::VoidKeyword);
        assert_eq!(lexer.file.len(), str.len());
        assert_eq!(lexer.tokens.len(), 1);
    }

    #[test]
    fn lexing_numbers() {

        let str = "123";
        let mut lexer = Lexer::new(str);
        lexer.lex();

        assert_eq!(lexer.tokens.front().unwrap().kind, SyntaxKind::NumberLiteral);
        assert_eq!(lexer.file.len(), str.len());
        assert_eq!(lexer.tokens.len(), 1);
        let token = lexer.tokens.pop_front().unwrap();
        assert_eq!(token.data, str);


        let str = "01293";
        let mut lexer = Lexer::new(str);
        lexer.lex();

        assert_eq!(lexer.tokens.front().unwrap().kind, SyntaxKind::NumberLiteral);
        assert_eq!(lexer.file.len(), str.len());
        assert_eq!(lexer.tokens.len(), 1);
        let token = lexer.tokens.pop_front().unwrap();
        assert_eq!(token.data, str);
    }


    // From this test, we should be able to conclude that all the locations are as expected!
    #[test]
    fn basic_symbols() {
        let str = "+ - * / ( \n ) { } = ==";
        // 1 2 3 4 5 6 7 8 9  10 11
        // +   -   *   /   (     \n
        //   )   {   }   =   ==
        let mut lexer = Lexer::new(str);
        lexer.lex();
        assert_eq!(lexer.file.len(), str.len());
        let mut token = lexer.tokens.pop_front().unwrap();
        assert_eq!(token.kind, SyntaxKind::Plus);
        assert_eq!(token.location, Location {
            line: 1,
            column: 1
        });

        token = lexer.tokens.pop_front().unwrap();
        assert_eq!(token.kind, SyntaxKind::Minus);
        assert_eq!(token.location, Location {
            line: 1,
            column: 3
        });

        token = lexer.tokens.pop_front().unwrap();
        assert_eq!(token.kind, SyntaxKind::Star);
        assert_eq!(token.location, Location {
            line: 1,
            column: 5
        });

        token = lexer.tokens.pop_front().unwrap();
        assert_eq!(token.kind, SyntaxKind::Slash);
        assert_eq!(token.location, Location {
            line: 1,
            column: 7
        });

        token = lexer.tokens.pop_front().unwrap();
        assert_eq!(token.kind, SyntaxKind::OpenParen);
        assert_eq!(token.location, Location {
            line: 1,
            column: 9
        });

        token = lexer.tokens.pop_front().unwrap();
        assert_eq!(token.kind, SyntaxKind::NewLine);
        assert_eq!(token.location, Location {
            line: 1,
            column: 11
        });

        token = lexer.tokens.pop_front().unwrap();
        assert_eq!(token.kind, SyntaxKind::CloseParen);
        assert_eq!(token.location, Location {
            line: 2,
            column: 2
        });

        token = lexer.tokens.pop_front().unwrap();
        assert_eq!(token.kind, SyntaxKind::OpenCurly);
        assert_eq!(token.location, Location {
            line: 2,
            column: 4
        });

        token = lexer.tokens.pop_front().unwrap();
        assert_eq!(token.kind, SyntaxKind::CloseCurly);
        assert_eq!(token.location, Location {
            line: 2,
            column: 6
        });

        token = lexer.tokens.pop_front().unwrap();
        assert_eq!(token.kind, SyntaxKind::Equal);
        assert_eq!(token.location, Location {
            line: 2,
            column: 8
        });

        token = lexer.tokens.pop_front().unwrap();
        assert_eq!(token.kind, SyntaxKind::EqualEqual);
        assert_eq!(token.location, Location {
            line: 2,
            column: 10
        });
    }
    #[test]
    fn basic_c_program() {
        let str = r#"
        int main(int argc, char **argv) {
            return 0;
        }
        "#;

        let mut lexer = Lexer::new(str);
        lexer.lex();
        assert_eq!(lexer.file.len(), str.len());
        assert_eq!(lexer.tokens.pop_front().unwrap().kind, SyntaxKind::NewLine);
        assert_eq!(lexer.tokens.pop_front().unwrap().kind, SyntaxKind::IntKeyword);
        assert_eq!(lexer.tokens.pop_front().unwrap().kind, SyntaxKind::StringLiteral);
        assert_eq!(lexer.tokens.pop_front().unwrap().kind, SyntaxKind::OpenParen);
        assert_eq!(lexer.tokens.pop_front().unwrap().kind, SyntaxKind::IntKeyword);
        assert_eq!(lexer.tokens.pop_front().unwrap().kind, SyntaxKind::StringLiteral);
        assert_eq!(lexer.tokens.pop_front().unwrap().kind, SyntaxKind::Comma);
        assert_eq!(lexer.tokens.pop_front().unwrap().kind, SyntaxKind::CharKeyword);
        assert_eq!(lexer.tokens.pop_front().unwrap().kind, SyntaxKind::Star);
        assert_eq!(lexer.tokens.pop_front().unwrap().kind, SyntaxKind::Star);
        assert_eq!(lexer.tokens.pop_front().unwrap().kind, SyntaxKind::StringLiteral);
        assert_eq!(lexer.tokens.pop_front().unwrap().kind, SyntaxKind::CloseParen);
        assert_eq!(lexer.tokens.pop_front().unwrap().kind, SyntaxKind::OpenCurly);
        assert_eq!(lexer.tokens.pop_front().unwrap().kind, SyntaxKind::NewLine);
        assert_eq!(lexer.tokens.pop_front().unwrap().kind, SyntaxKind::ReturnKeyword);
        assert_eq!(lexer.tokens.pop_front().unwrap().kind, SyntaxKind::NumberLiteral);
        assert_eq!(lexer.tokens.pop_front().unwrap().kind, SyntaxKind::Semicolon);
        assert_eq!(lexer.tokens.pop_front().unwrap().kind, SyntaxKind::NewLine);
        assert_eq!(lexer.tokens.pop_front().unwrap().kind, SyntaxKind::CloseCurly);
        assert_eq!(lexer.tokens.pop_front().unwrap().kind, SyntaxKind::NewLine);
    }
}