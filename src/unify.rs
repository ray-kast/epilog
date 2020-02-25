pub use crate::{
    prelude_internal::*,
    sub::{CowSub, Sub},
};
pub use std::{borrow::Cow, collections::HashMap};

pub trait Unify<K: Clone, V: Clone> {
    type Error;

    fn unify_with<'a>(&self, sub: CowSub<'a, K, V>, rhs: &Self) -> UResult<'a, K, V, Self::Error>;

    fn unify(&self, rhs: &Self) -> UResult<K, V, Self::Error> {
        self.unify_with(Cow::Owned(Sub::top()), rhs)
    }
}

// TODO: test me
impl<K: Clone, V: Clone, E, T: Unify<K, V, Error = E>> Unify<K, V> for Option<T> {
    type Error = E;

    fn unify_with<'a>(&self, sub: CowSub<'a, K, V>, rhs: &Self) -> UResult<'a, K, V, Self::Error> {
        match (self, rhs) {
            (Some(l), Some(r)) => l.unify_with(sub, r),
            (None, None) => UOk(sub),
            _ => Bottom,
        }
    }
}

// TODO: test me
impl<K: Clone, V: Clone, E, T: Unify<K, V, Error = E>> Unify<K, V> for Vec<T> {
    type Error = E;

    fn unify_with<'a>(
        &self,
        mut sub: CowSub<'a, K, V>,
        rhs: &Self,
    ) -> UResult<'a, K, V, Self::Error>
    {
        if self.len() != rhs.len() {
            return Bottom;
        }

        for (l, r) in self.iter().zip(rhs.iter()) {
            match l.unify_with(sub, r) {
                UOk(s) => sub = s,
                Bottom => return Bottom,
                UErr(e) => return UErr(e),
            }
        }

        UOk(sub)
    }
}

// TODO: test me
impl<K: Clone, V: Clone, E, K2: Unify<K, V, Error = E>, V2: Unify<K, V, Error = E>> Unify<K, V>
    for HashMap<K2, V2>
{
    type Error = E;

    fn unify_with<'a>(
        &self,
        mut sub: CowSub<'a, K, V>,
        rhs: &Self,
    ) -> UResult<'a, K, V, Self::Error>
    {
        if self.len() != rhs.len() {
            return Bottom;
        }

        for ((la, lb), (ra, rb)) in self.iter().zip(rhs.iter()) {
            match la.unify_with(sub, ra).and_then(|s| lb.unify_with(s, rb)) {
                UOk(s) => sub = s,
                Bottom => return Bottom,
                UErr(e) => return UErr(e),
            }
        }

        UOk(sub)
    }
}
