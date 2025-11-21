#[cfg(test)]
mod syntax_token_tests {
    use crate::lexer::syntax_token::*;

    #[test]
    fn testing_location() {
        let mut location = Location {
            line : 0,
            column : 0,
        };

        assert_eq!(location.line, 0);
        assert_eq!(location.column, 0);
        location.line += 1;
        location.column += 1;
        assert_eq!(location.line, 1);
        assert_eq!(location.column, 1);
        assert_eq!(location.line_and_column(), (1, 1));


        location.set_line_and_column(10, 13);
        assert_eq!(location.line, 10);
        assert_eq!(location.column, 13);
        assert_eq!(location.line_and_column(), (10, 13));
    }
    // We don't need to explicitly test the syntax token because it's being tested when we test the lexer.
}
