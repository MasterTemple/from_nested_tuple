extern crate from_nested_tuple_derive;

pub trait FromTuple {
    type Tuple;
    fn from_tuple(tuple: Self::Tuple) -> Self;
}
