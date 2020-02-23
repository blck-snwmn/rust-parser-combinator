/// ParseResult is parse result for practice
/// should use embedded Result
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseResult<T> {
    ///
    Success(T),
    Failure,
}

impl<T> ParseResult<T> {
    /// Returns ParseResult::Success
    /// ```
    /// use parser_combinator::ParseResult;
    /// assert_eq!(ParseResult::Success(10), ParseResult::success(10))
    /// ```
    pub fn success(v: T) -> Self {
        Self::Success(v)
    }
    /// Returns ParseResult::Failure
    /// ```
    /// use parser_combinator::ParseResult;
    /// assert_eq!(ParseResult::<()>::Failure, ParseResult::failure())
    /// ```
    pub fn failure() -> Self {
        Self::Failure
    }
}
