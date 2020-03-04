use std::collections::{HashMap, HashSet};

pub trait FreeVars<V> {
    fn free_vars_into<'a>(&'a self, set: &mut HashSet<&'a V>);

    fn free_vars(&self) -> HashSet<&V> {
        let mut set = HashSet::new();
        self.free_vars_into(&mut set);

        set
    }
}

pub trait FreshVars<V> {
    fn acquire(&self) -> V;
}

impl<V, T: FreshVars<V>> FreshVars<V> for &'_ T {
    fn acquire(&self) -> V { (*self).acquire() }
}

// TODO: unit test me
// TODO: property test me
impl<V, T: FreeVars<V>> FreeVars<V> for Option<T> {
    fn free_vars_into<'a>(&'a self, set: &mut HashSet<&'a V>) {
        match self {
            Some(ref v) => {
                v.free_vars_into(set);
            },
            None => (),
        }
    }
}

// TODO: unit test me
// TODO: property test me
impl<V, T: FreeVars<V>> FreeVars<V> for Vec<T> {
    fn free_vars_into<'a>(&'a self, set: &mut HashSet<&'a V>) {
        for el in self {
            el.free_vars_into(set);
        }
    }
}

// TODO: unit test me
// TODO: property test me
impl<V, K, W: FreeVars<V>> FreeVars<V> for HashMap<K, W> {
    fn free_vars_into<'a>(&'a self, set: &mut HashSet<&'a V>) {
        for val in self.values() {
            val.free_vars_into(set);
        }
    }
}
