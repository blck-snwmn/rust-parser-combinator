pub struct Parser<'a, T>
where
    T: 'a,
{
    func: Box<dyn 'a + Fn(&'a str) -> ParseResult<T>>,
}

impl<'a, T> Parser<'a, T>
where
    T: 'a,
{
    pub fn parse(&self, input: &'a str) -> ParseResult<T> {
        (self.func)(input)
    }

    pub fn map<S, F: 'a + Fn(T) -> S>(self, func: F) -> Parser<'a, S> {
        Parser {
            func: Box::new(move |v: &str| match self.parse(v) {
                ParseResult::Success(vv) => ParseResult::success(func(vv)),
                ParseResult::Failure => ParseResult::failure(),
            }),
        }
    }

    pub fn and_then<S, F: 'a + Fn(T) -> ParseResult<S>>(self, func: F) -> Parser<'a, S> {
        Parser {
            func: Box::new(move |v: &str| match self.parse(v) {
                ParseResult::Success(vv) => func(vv),
                ParseResult::Failure => ParseResult::failure(),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseResult<T> {
    Success(T),
    Failure,
}

impl<T> ParseResult<T> {
    fn success(v: T) -> Self {
        Self::Success(v)
    }
    fn failure() -> Self {
        Self::Failure
    }
}

#[test]
fn test_parse() {
    let p = Parser {
        func: Box::new(|s| ParseResult::success(s)),
    };
    let r = p.parse("a");
    assert_eq!(r, ParseResult::success("a"));
}

#[test]
fn test_parse_map() {
    let p = Parser {
        func: Box::new(|s| ParseResult::success(s)),
    };
    let p = p.map(|_| 10);
    let r = p.parse("a");
    assert_eq!(r, ParseResult::success(10));
}

#[test]
fn test_parse_and_then() {
    let p = Parser {
        func: Box::new(|s| ParseResult::success(s)),
    };
    let p = p.and_then(|_| ParseResult::success(15));
    let r = p.parse("a");
    assert_eq!(r, ParseResult::success(15));
}
