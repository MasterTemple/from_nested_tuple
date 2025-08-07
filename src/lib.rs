pub use from_nested_tuple_derive::FromTuple;

pub trait FromTuple {
    type Tuple;
    fn from_tuple(tuple: Self::Tuple) -> Self;
}
