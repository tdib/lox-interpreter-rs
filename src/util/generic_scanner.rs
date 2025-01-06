pub trait GenericScanner<T> {
    /// Has the scanner reached the last token to be scanned?
    fn is_at_end(&self) -> bool;

    /// Advance/consume a single (current) token, returning the consumed token
    fn consume(&mut self) -> T;

    /// Check if the current token is one of the expected, and if so, consume the token
    fn check_and_consume(&mut self, expected: &[T]) -> bool;

    /// Look at the current character without consuming
    fn peek(&self) -> T;

    /// Look at the next character without consuming
    fn peek_next(&self) -> T;
}
