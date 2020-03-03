pub(self) mod mocks;
pub(self) mod props;
pub(self) mod strats;
pub(self) mod units;

use crate::{ast::Error as AstError, prelude::*, sub::Sub};
use std::{
    borrow::Cow,
    cmp::Eq,
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub(self) fn set<T: Eq + Hash, I: IntoIterator<Item = T>>(iter: I) -> HashSet<T> {
    iter.into_iter().collect()
}

pub(self) fn dict<K: Eq + Hash, V, I: IntoIterator<Item = (K, V)>>(iter: I) -> HashMap<K, V> {
    iter.into_iter().collect()
}

pub(self) fn uok<K: Clone, V: Clone>(sub: Sub<K, V>) -> UResult<'static, K, V, AstError> {
    UOk(Cow::Owned(sub))
}

pub(self) fn top<K: Clone, V: Clone>() -> UResult<'static, K, V, AstError> { uok(Sub::top()) }

#[macro_export]
macro_rules! set { ($($tt:tt)*) => (crate::tests::set(vec![$($tt)*])); }

#[macro_export]
macro_rules! dict { ($($tt:tt)*) => (crate::tests::dict(vec![$($tt)*])); }

#[macro_export]
macro_rules! sub {
    ($($tt:tt)*) => {
        <crate::sub::Sub<_, _> as std::iter::FromIterator<_>>::from_iter(vec![$($tt)*])
    };
}

#[macro_export]
macro_rules! uok { ($($tt:tt)*) => (crate::tests::uok(sub![$($tt)*])); }
