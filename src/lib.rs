pub use from_nested_tuple_derive::FromTuple;

pub mod extension;
mod impls;

pub trait FromTuple {
    type Tuple;
    fn from_tuple(tuple: Self::Tuple) -> Self;
}
