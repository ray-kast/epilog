pub use crate::{
    prelude_internal::*,
    sub::{CowSub, Sub},
};
pub use std::{borrow::Cow, collections::HashMap};

// TODO: property test me for validity in unify tests
// TODO: property test me for commutativity in unify tests
// TODO: property test me for identity in unify tests
// TODO: property test me for unify_with vs .sub.unify in unify tests
pub trait Unify<K: Clone, V: Clone> {
    type Error;

    fn unify_with<'a>(&self, sub: CowSub<'a, K, V>, rhs: &Self) -> UResult<'a, K, V, Self::Error>;

    fn unify(&self, rhs: &Self) -> UResult<K, V, Self::Error> {
        self.unify_with(Cow::Owned(Sub::top()), rhs)
    }
}

// TODO: unit test me
// TODO: property test me
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

// TODO: unit test me
// TODO: property test me
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

// TODO: unit test me
// TODO: property test me
impl<K: Clone, V: Clone, E, K2: Eq + Hash, V2: Unify<K, V, Error = E>> Unify<K, V>
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

        for (l, r) in self.iter().map(|(k, v)| (v, rhs.get(k))) {
            match r {
                Some(r) => match l.unify_with(sub, r) {
                    UOk(s) => sub = s,
                    Bottom => return Bottom,
                    UErr(e) => return UErr(e),
                },
                None => return Bottom,
            }
        }

        UOk(sub)
    }
}
