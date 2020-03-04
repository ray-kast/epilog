use crate::ast;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum L {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum V {
    A,
    B,
    C,
    D,
}

pub use ast::{Error, VarSource};

pub type Ast = ast::Ast<L, V>;
pub type Term = ast::Term<L, V>;
pub type Var = ast::Var<V>;
