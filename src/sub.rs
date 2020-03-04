use crate::prelude_internal::*;
use std::{borrow::Cow, collections::HashMap, iter::FromIterator, rc::Rc};

#[cfg(feature = "try_trait")]
use std::ops::Try;

#[derive(Clone, Debug)]
pub struct Sub<K, V>(HashMap<K, Rc<V>>);

pub type CowSub<'a, K, V> = Cow<'a, Sub<K, V>>;

// TODO: avoid cloning if nothing changed
pub trait SubWith<K: Eq + Hash, V>: Sized {
    type Error;

    fn sub(&self, sub: &Sub<K, V>) -> Result<Self, Self::Error>;

    fn with(&self, key: K, val: V) -> Result<Self, Self::Error> { self.sub(&Sub::of(key, val)) }
}

impl<K, V> Sub<K, V> {
    pub fn top() -> Self { Self(HashMap::new()) }
}

impl<K: Eq + Hash, V> Sub<K, V> {
    pub fn of(key: K, val: V) -> Self {
        let mut map = HashMap::with_capacity(1);
        map.insert(key, Rc::new(val));

        Self(map)
    }

    pub fn get(&self, key: &K) -> Option<Rc<V>> { self.0.get(key).cloned() }
}

impl<K: Eq + Hash, V: PartialEq> PartialEq for Sub<K, V> {
    fn eq(&self, rhs: &Self) -> bool { self.0.eq(&rhs.0) }
}

impl<K: Eq + Hash, V: Eq> Eq for Sub<K, V> {}

impl<K, V> From<HashMap<K, Rc<V>>> for Sub<K, V> {
    fn from(map: HashMap<K, Rc<V>>) -> Self { Self(map) }
}

impl<K: Eq + Hash, V> From<HashMap<K, V>> for Sub<K, V> {
    fn from(map: HashMap<K, V>) -> Self { map.into_iter().collect() }
}

impl<K, V> From<Sub<K, V>> for HashMap<K, Rc<V>> {
    fn from(sub: Sub<K, V>) -> Self { sub.0 }
}

impl<K: Eq + Hash, V> FromIterator<(K, Rc<V>)> for Sub<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, Rc<V>)>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<K: Eq + Hash, V> FromIterator<(K, V)> for Sub<K, V> {
    #[inline] // because this just chains off the other FromIterator
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        iter.into_iter().map(|(k, v)| (k, Rc::new(v))).collect()
    }
}

impl<K: Clone + Eq + Hash, V: SubWith<K, V, Error = E>, E> SubWith<K, V> for Sub<K, V> {
    type Error = E;

    fn sub(&self, sub: &Sub<K, V>) -> Result<Self, Self::Error> {
        let mut map = HashMap::with_capacity(self.0.len() + sub.0.len());

        for (k, v) in &sub.0 {
            map.insert(k.clone(), v.clone());
        }

        for (k, v) in &self.0 {
            map.entry(k.clone()).or_insert(Rc::new(v.sub(sub)?));
        }

        Ok(Self(map))
    }
}

impl<'a, K: Clone + Eq + Hash, V: Clone + SubWith<K, V, Error = E>, E> SubWith<K, V>
    for CowSub<'a, K, V>
{
    type Error = E;

    fn sub(&self, sub: &Sub<K, V>) -> Result<Self, Self::Error> {
        self.as_ref().sub(sub).map(|s| Cow::Owned(s))
    }
}

impl<K: Eq + Hash, V, E, T: SubWith<K, V, Error = E>> SubWith<K, V> for Option<T> {
    type Error = E;

    fn sub(&self, sub: &Sub<K, V>) -> Result<Self, Self::Error> {
        match self {
            Some(v) => v.sub(sub).map(|v| Some(v)),
            None => Ok(None),
        }
    }
}

impl<K: Eq + Hash, V, E, T: SubWith<K, V, Error = E>> SubWith<K, V> for Vec<T> {
    type Error = E;

    fn sub(&self, sub: &Sub<K, V>) -> Result<Self, Self::Error> {
        self.into_iter().map(|e| e.sub(sub)).collect()
    }
}

impl<K: Eq + Hash, V, E, K2: Clone + Eq + Hash, V2: SubWith<K, V, Error = E>> SubWith<K, V>
    for HashMap<K2, V2>
{
    type Error = E;

    fn sub(&self, sub: &Sub<K, V>) -> Result<Self, Self::Error> {
        self.into_iter()
            .map(|(k, v)| v.sub(sub).map(|v| (k.clone(), v)))
            .collect()
    }
}
