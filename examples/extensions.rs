use chumsky::prelude::*;
use from_nested_tuple::{FromTuple, extension::ParserExt};

fn parser<'a>() -> impl Parser<'a, &'a str, (char, char, char)> {
    any().then(any()).then(any()).from_tuple()
}

fn main() {
    assert_eq!(parser().parse("abc").unwrap(), ('a', 'b', 'c'))
}
