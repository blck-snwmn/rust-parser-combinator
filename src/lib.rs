pub mod parser;
pub mod parser_result;

pub use crate::parser::Parser;
pub use crate::parser_result::ParseResult;

pub fn match_string<'a>(patten: &str) -> Parser<'a, &str> {
    Parser {
        func: Box::new(move |input: &str| {
            if input.starts_with(patten) {
                ParseResult::success(patten)
            } else {
                ParseResult::failure()
            }
        }),
    }
}
#[test]
fn test_match_string() {
    let p = match_string("+");
    assert_eq!(p.parse("s"), ParseResult::failure());
    assert_eq!(p.parse("+"), ParseResult::success("+"));
    assert_eq!(p.parse("="), ParseResult::failure());

    let p = match_string("abc");
    assert_eq!(p.parse("abe"), ParseResult::failure());
    assert_eq!(p.parse("abcd"), ParseResult::success("abc"));
    assert_eq!(p.parse("x"), ParseResult::failure());
}
