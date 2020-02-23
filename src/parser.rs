use crate::parser_result::ParseResult;

pub struct Parser<'a, T>
where
    T: 'a,
{
    pub func: Box<dyn 'a + Fn(&'a str) -> ParseResult<T>>,
}

impl<'a, T> Parser<'a, T>
where
    T: 'a,
{
    /// Parses input using Parser.func
    ///
    /// ```
    /// use parser_combinator::*;
    ///
    /// let p = Parser {
    ///     func: Box::new(|s| ParseResult::success(s)),
    /// };
    /// assert_eq!(p.parse("a"), ParseResult::success("a"));
    /// ```
    pub fn parse(&self, input: &'a str) -> ParseResult<T> {
        (self.func)(input)
    }

    /// Maps an `Parser<T>` to `Parser<S>` by applying a function to a contained value
    ///
    /// ```
    /// use parser_combinator::*;
    ///
    /// let p = Parser {
    ///     func: Box::new(|s| ParseResult::success(s)),
    /// };
    /// let p = p.map(|s| s.len());
    /// assert_eq!(p.parse("aaaa"), ParseResult::success(4));
    /// ```
    pub fn map<S, F: 'a + Fn(T) -> S>(self, func: F) -> Parser<'a, S> {
        Parser {
            func: Box::new(move |v: &str| match self.parse(v) {
                ParseResult::Success(vv) => ParseResult::success(func(vv)),
                ParseResult::Failure => ParseResult::failure(),
            }),
        }
    }

    /// Returns Parser to return [`Failure`] if ParseResult is [`Failure`],
    /// otherwise return result to apply `f` to contained value.
    ///
    /// ```
    /// use parser_combinator::*;
    ///
    /// let p = Parser {
    ///     func: Box::new(|s| ParseResult::success(s)),
    /// };
    /// let p = p.and_then(|s| ParseResult::success(s.len()));
    /// assert_eq!(p.parse("abcdefg"), ParseResult::success(7));
    /// ```
    pub fn and_then<S, F: 'a + Fn(T) -> ParseResult<S>>(self, func: F) -> Parser<'a, S> {
        Parser {
            func: Box::new(move |v: &str| match self.parse(v) {
                ParseResult::Success(vv) => func(vv),
                ParseResult::Failure => ParseResult::failure(),
            }),
        }
    }
}
