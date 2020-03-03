use super::{lit::Lit, var};
use crate::ast;

pub use ast::{Error, VarSource};

pub type Ast = ast::Ast<Lit, var::Var>;
pub type Term = ast::Term<Lit, var::Var>;
pub type Var = ast::Var<var::Var>;
