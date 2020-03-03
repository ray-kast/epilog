#![cfg_attr(test, feature(try_trait))]

pub mod ast;
pub mod quant;
pub mod sub;
pub mod unify;
pub mod uresult;
pub mod var;

#[cfg(test)]
mod tests;

mod prelude_internal {
    pub use crate::uresult::prelude::*;
    pub use std::{cmp::Eq, hash::Hash};
}

pub mod prelude {
    pub use crate::{
        sub::SubWith,
        unify::Unify,
        uresult::prelude::*,
        var::{FreeVars, FreshVars},
    };
}
