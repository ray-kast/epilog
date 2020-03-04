use crate::{sub, sub::SubWith};
use proptest::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Val {
    Lit(u32),
    Var(u32),
}

pub use Val::*;

impl SubWith<u32, Val> for Val {
    type Error = ();

    fn sub(&self, sub: &Sub) -> Result<Self, Self::Error> {
        Ok(match self {
            Self::Lit(l) => Self::Lit(*l),
            Self::Var(v) => match sub.get(v) {
                Some(v) => *v,
                None => Self::Var(*v),
            },
        })
    }
}

pub type Sub = sub::Sub<u32, Val>;

pub fn val(size: u32) -> impl Strategy<Value = Val> {
    prop_oneof![(0..size).prop_map(Val::Lit), (0..size).prop_map(Val::Var)]
}

pub fn sub(size: u32) -> impl Strategy<Value = Sub> {
    prop::collection::hash_map(0..size, val(size), 0..size as usize).prop_map(Sub::from)
}
