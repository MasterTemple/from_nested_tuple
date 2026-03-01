pub use from_nested_tuple_derive::FromTuple;

mod impls;

pub mod extension;
pub use extension::FromTupleExt as FromTuple;

pub trait FromNestedTuple {
    type Tuple;
    fn from_nested_tuple(tuple: Self::Tuple) -> Self;
}
