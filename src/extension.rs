use super::FromNestedTuple;
use chumsky::prelude::*;

pub trait FromTupleExt<'src, I, O, E>
where
    Self: Parser<'src, I, O, E>,
    I: Input<'src>,
    E: extra::ParserExtra<'src, I>,
{
    fn from_tuple<N>(self) -> impl Parser<'src, I, N, E>
    where
        Self: Sized,
        N: FromNestedTuple<Tuple = O>,
    {
        self.map(FromNestedTuple::from_nested_tuple)
    }
}

impl<'src, I, O, E, T> FromTupleExt<'src, I, O, E> for T
where
    T: Parser<'src, I, O, E>,
    I: Input<'src>,
    E: extra::ParserExtra<'src, I>,
{
    fn from_tuple<N>(self) -> impl Parser<'src, I, N, E>
    where
        Self: Sized,
        N: FromNestedTuple<Tuple = O>,
    {
        self.map(FromNestedTuple::from_nested_tuple)
    }
}
