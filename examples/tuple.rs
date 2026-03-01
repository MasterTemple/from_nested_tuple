use chumsky::prelude::*;
use from_nested_tuple::FromTuple;

fn parser<'a>() -> impl Parser<'a, &'a str, (char, char, char)> {
    any().then(any()).then(any()).map(FromTuple::from_tuple)
}

fn main() {
    assert_eq!(parser().parse("abc").unwrap(), ('a', 'b', 'c'))
}
