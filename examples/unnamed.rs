use chumsky::prelude::*;
use from_nested_tuple::FromTuple;

#[derive(Debug, PartialEq, Eq, FromTuple)]
struct WithUnnamedFields(char, char, char);

impl WithUnnamedFields {
    fn parser<'a>() -> impl Parser<'a, &'a str, Self> {
        any().then(any()).then(any()).map(FromTuple::from_tuple)
    }
}

fn main() {
    assert_eq!(
        WithUnnamedFields::parser().parse("abc").unwrap(),
        WithUnnamedFields('a', 'b', 'c')
    );
}
