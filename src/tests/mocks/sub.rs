use crate::{prelude::*, sub::Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Val {
    Lit(&'static str),
    Var(&'static str),
}

pub use Val::*;

impl SubWith<&'static str, Val> for Val {
    type Error = ();

    fn sub(&self, sub: &Sub<&'static str, Val>) -> Result<Self, Self::Error> {
        Ok(match self {
            Self::Lit(s) => Self::Lit(s),
            Self::Var(v) => match sub.get(v) {
                Some(v) => *v,
                None => Self::Var(v),
            },
        })
    }
}
