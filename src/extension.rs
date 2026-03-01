use super::FromTuple;
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
        N: FromTuple<Tuple = O>,
    {
        self.map(FromTuple::from_tuple)
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
        N: FromTuple<Tuple = O>,
    {
        self.map(FromTuple::from_tuple)
    }
}
