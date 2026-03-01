use chumsky::prelude::*;
use from_nested_tuple::FromTuple;

#[derive(Debug, PartialEq, Eq, FromTuple)]
struct WithNamedFields {
    a: char,
    b: char,
    c: char,
}

impl WithNamedFields {
    fn parser<'a>() -> impl Parser<'a, &'a str, Self> {
        any().then(any()).then(any()).map(FromTuple::from_tuple)
    }
}

fn main() {
    assert_eq!(
        WithNamedFields::parser().parse("123").unwrap(),
        WithNamedFields {
            a: '1',
            b: '2',
            c: '3'
        }
    );
}
