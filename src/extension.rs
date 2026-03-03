use super::FromNestedTuple;
use chumsky::{
    extension::v1::{Ext, ExtParser},
    extra::ParserExtra,
    input::InputRef,
    prelude::*,
};

/// This allegedly does things that PhantomData doesn't
/// It is taken directly from chumsky, but it is here because it is not publicly exposed
mod empty_phantom {
    /// A type that allows mentioning type parameters *without* all of the customary omission of auto traits that comes
    /// with `PhantomData`.
    pub(super) struct EmptyPhantom<T>(core::marker::PhantomData<T>);

    impl<T> EmptyPhantom<T> {
        pub(super) const fn new() -> Self {
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
}
use empty_phantom::EmptyPhantom;

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
    }
}

// Expose `from_tuple` as a chain-able method

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
            _new: EmptyPhantom::new(),
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
            _new: EmptyPhantom::new(),
        })
    }
}
