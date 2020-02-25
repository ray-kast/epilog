pub mod ast;
pub mod sub;
pub mod unify;
pub mod uresult;

mod prelude_internal {
    pub use crate::uresult::prelude::*;
    pub use std::{cmp::Eq, hash::Hash};
}

pub mod prelude {
    pub use crate::{sub::SubWith, unify::Unify, uresult::prelude::*};
}
