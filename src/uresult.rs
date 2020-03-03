use crate::{prelude_internal::*, sub::CowSub};

#[cfg(any(test, feature = "try_trait"))]
use std::ops::Try;

pub mod prelude {
    pub use super::{
        UResult,
        UResult::{Bottom, Err as UErr, Ok as UOk},
    };
}

#[derive(Clone, Debug)]
pub enum UResult<'a, K: Clone, V: Clone, E> {
    Ok(CowSub<'a, K, V>),
    Bottom,
    Err(E),
}

// TODO: unit test me
// TODO: property test me
impl<'a, K: Clone, V: Clone, E> UResult<'a, K, V, E> {
    pub fn and_then<K2: Clone, V2: Clone, F: FnOnce(CowSub<'a, K, V>) -> UResult<'a, K2, V2, E>>(
        self,
        op: F,
    ) -> UResult<'a, K2, V2, E>
    {
        match self {
            Self::Ok(s) => op(s),
            Self::Bottom => Bottom,
            Self::Err(e) => UErr(e),
        }
    }
}

// TODO: unit test me
// TODO: property test me
#[cfg(any(test, feature = "try_trait"))]
impl<'a, K: Clone, V: Clone, E> Try for UResult<'a, K, V, E> {
    type Error = E;
    type Ok = Option<CowSub<'a, K, V>>;

    #[inline]
    fn into_result(self) -> Result<<Self as Try>::Ok, Self::Error> { self.into() }

    #[inline]
    fn from_ok(sub: <Self as Try>::Ok) -> Self {
        match sub {
            Some(s) => UOk(s),
            None => Bottom,
        }
    }

    #[inline]
    fn from_error(err: E) -> Self { UErr(err) }
}

// TODO: unit test me
// TODO: property test me
impl<'a, K: Clone + Eq + Hash, V: Clone + PartialEq, E: PartialEq> PartialEq<UResult<'a, K, V, E>>
    for UResult<'a, K, V, E>
{
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Self::Ok(l), Self::Ok(r)) => l.eq(r),
            (Self::Bottom, Self::Bottom) => true,
            (Self::Err(l), Self::Err(r)) => l.eq(r),
            (_, _) => false,
        }
    }
}

impl<'a, K: Clone + Eq + Hash, V: Clone + Eq, E: Eq> Eq for UResult<'a, K, V, E> {}

// TODO: unit test me
// TODO: property test me
impl<'a, K: Clone, V: Clone, E> From<Result<CowSub<'a, K, V>, E>> for UResult<'a, K, V, E> {
    fn from(res: Result<CowSub<'a, K, V>, E>) -> Self {
        match res {
            Ok(s) => Self::Ok(s),
            Err(e) => Self::Err(e),
        }
    }
}

// TODO: unit test me
// TODO: property test me
impl<'a, K: Clone, V: Clone, E> From<Result<Option<CowSub<'a, K, V>>, E>> for UResult<'a, K, V, E> {
    fn from(res: Result<Option<CowSub<'a, K, V>>, E>) -> Self {
        match res {
            Ok(Some(s)) => Self::Ok(s),
            Ok(None) => Self::Bottom,
            Err(e) => Self::Err(e),
        }
    }
}

// TODO: unit test me
// TODO: property test me
impl<'a, K: Clone, V: Clone, E> From<UResult<'a, K, V, E>> for Result<Option<CowSub<'a, K, V>>, E> {
    fn from(res: UResult<'a, K, V, E>) -> Self {
        match res {
            UOk(s) => Ok(Some(s)),
            Bottom => Ok(None),
            UErr(e) => Err(e),
        }
    }
}
