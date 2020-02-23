#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseResult<T> {
    Success(T),
    Failure,
}

impl<T> ParseResult<T> {
    pub fn success(v: T) -> Self {
        Self::Success(v)
    }
    pub fn failure() -> Self {
        Self::Failure
    }
}
