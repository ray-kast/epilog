use crate::{
    prelude_internal::*,
    sub::{CowSub, Sub, SubWith},
    unify::Unify,
};
use std::{collections::HashMap, convert::TryFrom};

#[derive(Clone, Debug)]
pub enum Ast<L, V> {
    Lit(L),
    Var(Var<V>),
    List(Term<L, V>, Vec<Ast<L, V>>),
    /// NOTE: do NOT rely on unification order for this
    Dict(Term<L, V>, HashMap<Term<L, V>, Ast<L, V>>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Term<L, V> {
    Lit(L),
    Var(Var<V>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Var<V> {
    User(V),
    Auto(u64),
}

pub enum Error {
    BadTermSub, // Attempt to substitute an Ast for a Term
}

impl<L, V> From<Term<L, V>> for Ast<L, V> {
    fn from(term: Term<L, V>) -> Self {
        match term {
            Term::Lit(l) => Ast::Lit(l),
            Term::Var(v) => Ast::Var(v),
        }
    }
}

impl<L, V> TryFrom<Ast<L, V>> for Term<L, V> {
    type Error = Ast<L, V>;

    fn try_from(ast: Ast<L, V>) -> Result<Self, Self::Error> {
        match ast {
            Ast::Lit(l) => Ok(Term::Lit(l)),
            Ast::Var(v) => Ok(Term::Var(v)),
            a => Err(a),
        }
    }
}

// TODO: test me
impl<L: Clone + Eq + Hash, V: Clone + Eq + Hash> SubWith<Var<V>, Ast<L, V>> for Ast<L, V> {
    type Error = Error;

    fn sub(&self, sub: &Sub<Var<V>, Ast<L, V>>) -> Result<Self, Self::Error> {
        match self {
            Self::Lit(l) => Ok(Self::Lit(l.clone())),
            Self::Var(v) => Ok(match sub.get(&v) {
                Some(a) => a.as_ref().clone(),
                None => Self::Var(v.clone()),
            }),
            Self::List(t, l) => t
                .sub(sub)
                .and_then(|t| l.sub(sub).map(|l| Self::List(t, l))),
            Self::Dict(t, d) => t
                .sub(sub)
                .and_then(|t| d.sub(sub).map(|d| Self::Dict(t, d))),
        }
    }
}

// TODO: test me
impl<L: Clone + Eq + Hash, V: Clone + Eq + Hash> Unify<Var<V>, Ast<L, V>> for Ast<L, V> {
    type Error = Error;

    fn unify_with<'a>(
        &self,
        sub: CowSub<'a, Var<V>, Ast<L, V>>,
        rhs: &Self,
    ) -> UResult<'a, Var<V>, Ast<L, V>, Self::Error>
    {
        match (self, rhs) {
            (Self::Lit(l), Self::Lit(r)) if l == r => UOk(sub),
            (Self::Var(l), r) => match sub.get(l) {
                Some(a) => a.unify_with(sub, rhs),
                None => sub.with(l.clone(), r.clone()).into(),
            },
            (l, Self::Var(r)) => match sub.get(r) {
                Some(a) => l.unify_with(sub, a.as_ref()),
                None => sub.with(r.clone(), l.clone()).into(),
            },
            (Self::List(lt, ll), Self::List(rt, rl)) => {
                lt.unify_with(sub, rt).and_then(|s| ll.unify_with(s, rl))
            },
            (Self::Dict(lt, ld), Self::Dict(rt, rd)) => {
                lt.unify_with(sub, rt).and_then(|s| ld.unify_with(s, rd))
            },
            _ => Bottom,
        }
    }
}

// TODO: test me
impl<L: Clone, V: Clone + Eq + Hash> SubWith<Var<V>, Ast<L, V>> for Term<L, V> {
    type Error = Error;

    fn sub(&self, sub: &Sub<Var<V>, Ast<L, V>>) -> Result<Self, Self::Error> {
        match self {
            Self::Lit(l) => Ok(Self::Lit(l.clone())),
            Self::Var(v) => match sub.get(&v) {
                Some(r) => match r.as_ref() {
                    Ast::Lit(l) => Ok(Term::Lit(l.clone())),
                    Ast::Var(v) => Ok(Term::Var(v.clone())),
                    _ => Err(Error::BadTermSub),
                },
                None => Ok(Self::Var(v.clone())),
            },
        }
    }
}

// TODO: test me
impl<L: Clone + Eq + Hash, V: Clone + Eq + Hash> Unify<Var<V>, Ast<L, V>> for Term<L, V> {
    type Error = Error;

    fn unify_with<'a>(
        &self,
        sub: CowSub<'a, Var<V>, Ast<L, V>>,
        rhs: &Self,
    ) -> UResult<'a, Var<V>, Ast<L, V>, Self::Error>
    {
        match (self, rhs) {
            (Term::Lit(l), Term::Lit(r)) if l == r => UOk(sub),
            (Term::Var(l), r) => match sub.get(l) {
                Some(a) => match a.as_ref() {
                    Ast::Lit(l) => Term::Lit(l.clone()).unify_with(sub, rhs),
                    Ast::Var(v) => Term::Var(v.clone()).unify_with(sub, rhs),
                    _ => UErr(Error::BadTermSub),
                },
                None => sub.with(l.clone(), r.clone().into()).into(),
            },
            (l, Term::Var(r)) => match sub.get(r) {
                Some(a) => match a.as_ref() {
                    Ast::Lit(i) => l.unify_with(sub, &Term::Lit(i.clone())),
                    Ast::Var(v) => l.unify_with(sub, &Term::Var(v.clone())),
                    _ => UErr(Error::BadTermSub),
                },
                None => sub.with(r.clone(), l.clone().into()).into(),
            },
            _ => Bottom,
        }
    }
}
