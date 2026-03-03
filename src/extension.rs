use std::marker::PhantomData;

use super::FromNestedTuple;
use chumsky::{
    extension::v1::{Ext, ExtParser},
    extra::ParserExtra,
    input::InputRef,
    prelude::*,
};

/// A type that allows mentioning type parameters *without* all of the customary omission of auto traits that comes
/// with `PhantomData`.
struct EmptyPhantom<T>(core::marker::PhantomData<T>);

impl<T> EmptyPhantom<T> {
    const fn new() -> Self {
        Self(core::marker::PhantomData)
    }
}

impl<T> Copy for EmptyPhantom<T> {}
impl<T> Clone for EmptyPhantom<T> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: This is safe because `EmptyPhantom` doesn't actually contain a `T`.
unsafe impl<T> Send for EmptyPhantom<T> {}
// SAFETY: This is safe because `EmptyPhantom` doesn't actually contain a `T`.
unsafe impl<T> Sync for EmptyPhantom<T> {}
impl<T> Unpin for EmptyPhantom<T> {}
impl<T> core::panic::UnwindSafe for EmptyPhantom<T> {}
impl<T> core::panic::RefUnwindSafe for EmptyPhantom<T> {}

// pub struct MapFromTuple<A, O, OA>
// where
//     OA: FromNestedTuple<Tuple = O>,
// {
//     pub(crate) parser: A,
//     #[allow(dead_code)]
//     pub(crate) _o: EmptyPhantom<O>,
//     pub(crate) _a: EmptyPhantom<OA>,
// }
//
// impl<A: Copy, O, OA> Copy for MapFromTuple<A, O, OA> where OA: FromNestedTuple<Tuple = O> {}
// impl<A: Clone, O, OA> Clone for MapFromTuple<A, O, OA>
// where
//     OA: FromNestedTuple<Tuple = O>,
// {
//     fn clone(&self) -> Self {
//         Self {
//             parser: self.parser.clone(),
//             _o: EmptyPhantom::new(),
//             _a: EmptyPhantom::new(),
//         }
//     }
// }

// impl<'src, I, O, E, A, OA> Parser<'src, I, O, E> for MapFromTuple<A, OA>
// where
//     I: Input<'src>,
//     E: ParserExtra<'src, I>,
//     A: Parser<'src, I, OA, E>,
// {
//     #[doc(hidden)]
//     #[cfg(feature = "debug")]
//     fn node_info(&self, scope: &mut debug::NodeScope) -> debug::NodeInfo {
//         self.parser.node_info(scope)
//     }
//
//     #[inline(always)]
//     fn go<M: Mode>(&self, inp: &mut InputRef<'src, '_, I, E>) -> PResult<M, O> {
//         let out = self.parser.go::<M>(inp)?;
//         Ok(M::map(out, &FromNestedTuple::from_nested_tuple))
//     }
//
//     go_extra!(O);
// }

// impl<'src, I, O, E, A, OA> ExtParser<'src, I, OA, E> for MapFromTuple<A, O, OA>
// where
//     A: Parser<'src, I, O, E>,
//     OA: FromNestedTuple<Tuple = O>,
//     I: Input<'src, Token = u8>,
//     E: extra::ParserExtra<'src, I>,
// {
//     fn parse(&self, inp: &mut InputRef<'src, '_, I, E>) -> Result<OA, E::Error> {
//         // self.parser.map(f)
//         todo!()
//     }
// }

// impl<'src, I, E, A, OA> ExtParser<'src, I, (), E> for MapFromTuple<A, OA>
// where
//     A: Parser<'src, I, A, E>,
//     OA: FromNestedTuple<Tuple = A>,
//     I: Input<'src, Token = u8>,
//     E: extra::ParserExtra<'src, I>,
// {
//     fn parse(&self, inp: &mut InputRef<'src, '_, I, E>) -> impl Parser<'src, I, OA, E> {
//         self.parser.map(FromNestedTuple::from_nested_tuple)
//     }
// }
//

pub type MapFromTuple<A, N> = Ext<MapFromTuple_<A, N>>;

pub struct MapFromTuple_<A, N> {
    pub(crate) parser: A,
    _new: EmptyPhantom<N>,
}

impl<A: Copy, N> Copy for MapFromTuple_<A, N> {}
impl<A: Clone, N> Clone for MapFromTuple_<A, N> {
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            _new: self._new.clone(),
        }
    }
}

impl<'src, I, O, E, A, N> ExtParser<'src, I, N, E> for MapFromTuple_<A, N>
where
    I: Input<'src>,
    E: ParserExtra<'src, I>,
    A: Parser<'src, I, O, E>,
    N: FromNestedTuple<Tuple = O>,
{
    fn parse(
        &self,
        inp: &mut InputRef<'src, '_, I, E>,
    ) -> Result<N, <E as ParserExtra<'src, I>>::Error> {
        inp.parse((&self.parser).map(FromNestedTuple::from_nested_tuple))
        // self.parser
        //     .map(FromNestedTuple::from_nested_tuple)
        //     .parse(inp)
    }
}

// impl<'src, I, N, E> Parser<'src, I, N, E> for Map2 {
//     fn go<M: Mode>(&self, inp: &mut InputRef<'src, '_, I, E>) -> PResult<M, N>
//     where
//         Self: Sized,
//     {
//         let out = self.parser.go::<M>(inp)?;
//         Ok(M::map(out, &FromNestedTuple::from_nested_tuple))
//     }
//
//     fn go_emit(&self, inp: &mut InputRef<'src, '_, I, E>) -> PResult<Emit, N> {
//         todo!()
//     }
//
//     fn go_check(&self, inp: &mut InputRef<'src, '_, I, E>) -> PResult<Check, N> {
//         todo!()
//     }
// }

pub trait FromTupleExt<'src, I, O, E>
where
    Self: Parser<'src, I, O, E> + Clone,
    I: Input<'src>,
    E: extra::ParserExtra<'src, I>,
{
    fn from_tuple<N>(self) -> MapFromTuple<Self, N>
    where
        Self: Sized,
        N: FromNestedTuple<Tuple = O>,
    {
        Ext(MapFromTuple_ {
            parser: self,
            _new: EmptyPhantom(PhantomData),
        })
    }
}

impl<'src, I, O, E, T> FromTupleExt<'src, I, O, E> for T
where
    Self: Parser<'src, I, O, E> + Clone,
    I: Input<'src>,
    E: extra::ParserExtra<'src, I>,
{
    fn from_tuple<N>(self) -> MapFromTuple<Self, N>
    where
        Self: Sized,
        N: FromNestedTuple<Tuple = O>,
    {
        Ext(MapFromTuple_ {
            parser: self,
            _new: EmptyPhantom(PhantomData),
        })
    }
}

// impl<'src, I, O, E, T> FromTupleExt<'src, I, O, E> for T
// where
//     T: Parser<'src, I, O, E> + Clone,
//     I: Input<'src>,
//     E: extra::ParserExtra<'src, I>,
// {
//     fn from_tuple<N>(self) -> impl Parser<'src, I, N, E> + Clone
//     where
//         Self: Sized,
//         N: FromNestedTuple<Tuple = O>,
//     {
//         self.map(FromNestedTuple::from_nested_tuple)
//     }
// }

// pub trait FromTupleExt<'src, I, O, E>
// where
//     Self: Parser<'src, I, O, E> + Clone,
//     I: Input<'src>,
//     E: extra::ParserExtra<'src, I>,
// {
//     fn from_tuple<N>(self) -> impl Parser<'src, I, N, E> + Clone
//     where
//         Self: Sized,
//         N: FromNestedTuple<Tuple = O>,
//     {
//         self.map(FromNestedTuple::from_nested_tuple)
//     }
// }
//
// impl<'src, I, O, E, T> FromTupleExt<'src, I, O, E> for T
// where
//     T: Parser<'src, I, O, E> + Clone,
//     I: Input<'src>,
//     E: extra::ParserExtra<'src, I>,
// {
//     fn from_tuple<N>(self) -> impl Parser<'src, I, N, E> + Clone
//     where
//         Self: Sized,
//         N: FromNestedTuple<Tuple = O>,
//     {
//         self.map(FromNestedTuple::from_nested_tuple)
//     }
// }
