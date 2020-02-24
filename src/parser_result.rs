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

    /// Maps an `ParseResult<T>` to `ParseResult<S>` by applying a function to a contained value
    ///
    /// ```
    /// use parser_combinator::ParseResult;
    ///
    /// let r = ParseResult::Success("abc");
    /// let r = r.map(|s|s.len());
    /// assert_eq!(r, ParseResult::success(3))
    /// ```
    pub fn map<S, F: Fn(T) -> S>(self, f: F) -> ParseResult<S> {
        use self::ParseResult::*;

        match self {
            Success(v) => Success(f(v)),
            Failure => Failure,
        }
    }

    /// Returns [`Failure`] if ParseResult is [`Failure`],
    /// otherwise return result to apply `f` to contained value
    ///
    /// ```
    /// use parser_combinator::ParseResult;
    ///
    /// let r = ParseResult::Success("abcd");
    /// let r = r.and_then(|s| ParseResult::Success(s.len()));
    /// assert_eq!(r, ParseResult::success(4))
    /// ```
    pub fn and_then<S, F: Fn(T) -> ParseResult<S>>(self, f: F) -> ParseResult<S> {
        use self::ParseResult::*;

        match self {
            Success(v) => f(v),
            Failure => Failure,
        }
    }
}
