use crate::{ast, sub};
use proptest::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct L(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct V(pub u32);

pub type Ast = ast::Ast<L, V>;
pub type Term = ast::Term<L, V>;
pub type Var = ast::Var<V>;

pub type Sub = sub::Sub<Var, Ast>;

pub fn ast(levels: u32, size: u32, fanout: u32) -> impl Strategy<Value = Ast> {
    let leaf = term(size).prop_map(Ast::from);

    leaf.prop_recursive(levels, size, fanout, move |inner| {
        prop_oneof![
            (
                term(size),
                prop::collection::vec(inner.clone(), 0..fanout as usize)
            )
                .prop_map(|(t, v)| Ast::List(t, v)),
            (
                term(size),
                prop::collection::hash_map((0..size).prop_map(L), inner, 0..fanout as usize)
            )
                .prop_map(|(t, d)| Ast::Dict(t, d)),
        ]
    })
}

pub fn term(size: u32) -> impl Strategy<Value = Term> {
    prop_oneof![
        2 => (0..size).prop_map(|l| Term::Lit(L(l))),
        1 => (0..size).prop_map(|v| Term::Var(Var::User(V(v)))),
        1 => (0..size).prop_map(|v| Term::Var(Var::Auto(v))),
    ]
}
// TODO: this may be parameterizable
pub fn sub(levels: u32, size: u32, fanout: u32) -> impl Strategy<Value = Sub> {
    prop::collection::hash_map(
        prop_oneof![
            (0..size).prop_map(|v| Var::User(V(v))),
            (0..size).prop_map(|v| Var::Auto(v)),
        ],
        ast(levels, size, fanout),
        0..size as usize,
    )
    .prop_map(Sub::from)
}
