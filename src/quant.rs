use crate::{
    prelude_internal::*,
    sub::SubWith,
    var::{FreeVars, FreshVars},
};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct For<V, T>(HashSet<V>, T);

impl<V, T> For<V, T> {
    pub fn gen_on(on: HashSet<V>, t: T) -> Self { Self(on, t) }

    pub fn val(&self) -> &T { &self.1 }
}

// TODO: unit test me
// TODO: property test me
impl<V: Clone + Eq + Hash, T: FreeVars<V>> For<V, T> {
    pub fn gen(t: T) -> Self { Self::gen_on(t.free_vars().into_iter().cloned().collect(), t) }
}

// TODO: unit test me
// TODO: property test me
impl<V: Clone + Eq + Hash, T> For<V, T> {
    pub fn inst<W, S: FreshVars<W>>(&self, src: S) -> Result<T, T::Error>
    where T: SubWith<V, W> {
        self.1
            .sub(&self.0.iter().cloned().map(|v| (v, src.acquire())).collect())
    }
}
