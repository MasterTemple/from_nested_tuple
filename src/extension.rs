use chumsky::prelude::*;

pub trait ParserExt<'src, I, O, E>
where
    Self: Parser<'src, I, O, E>,
    I: Input<'src>,
    E: extra::ParserExtra<'src, I>,
{
    fn from_tuple<N>(self) -> impl Parser<'src, I, N, E>
    where
        Self: Sized,
        N: super::FromTuple<Tuple = O>,
    {
        self.map(super::FromTuple::from_tuple)
    }
}

impl<'src, I, O, E, T> ParserExt<'src, I, O, E> for T
where
    T: Parser<'src, I, O, E>,
    I: Input<'src>,
    E: extra::ParserExtra<'src, I>,
{
    fn from_tuple<N>(self) -> impl Parser<'src, I, N, E>
    where
        Self: Sized,
        N: super::FromTuple<Tuple = O>,
    {
        self.map(super::FromTuple::from_tuple)
    }
}
