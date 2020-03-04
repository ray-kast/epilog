use crate::{
    dict,
    prelude::*,
    set, sub,
    tests::{mocks::ast::*, top},
    uok,
};

#[test]
fn freevars_ast() {
    let va = Var::User(V::A);
    let vb = Var::User(V::B);
    let vc = Var::User(V::C);
    let vd = Var::User(V::D);

    assert_eq!(Ast::Lit(L::A).free_vars(), set![]);

    assert_eq!(Ast::Var(va).free_vars(), set![&va]);

    assert_eq!(
        Ast::List(Term::Lit(L::A), vec![va.into(), vb.into(), vc.into()]).free_vars(),
        set![&va, &vb, &vc]
    );

    assert_eq!(
        Ast::List(va.into(), vec![vb.into(), vc.into()]).free_vars(),
        set![&va, &vb, &vc]
    );

    assert_eq!(
        Ast::Dict(
            Term::Lit(L::A),
            dict![(L::A, va.into()), (L::B, vc.into()), (L::C, vb.into())],
        )
        .free_vars(),
        set![&va, &vb, &vc],
    );

    assert_eq!(
        Ast::Dict(
            vd.into(),
            dict![(L::D, va.into()), (L::A, vb.into()), (L::C, vc.into())],
        )
        .free_vars(),
        set![&va, &vb, &vc, &vd],
    )
}

#[test]
fn freevars_term() {
    let va = Var::User(V::A);

    assert_eq!(Term::Lit(L::A).free_vars(), set![]);
    assert_eq!(Term::Var(va).free_vars(), set![&va]);
}

#[test]
fn freevars_var() {
    let va = Var::User(V::A);

    assert_eq!(va.free_vars(), set![&va]);
}

#[test]
fn sub_ast() {
    let va = Var::User(V::A);
    let vb = Var::User(V::B);
    let vc = Var::User(V::C);
    let vd = Var::User(V::D);

    for lit in vec![L::A, L::B, L::C, L::D] {
        assert_eq!(
            Ast::Lit(lit).sub(&sub![
                (vd, Ast::Lit(L::A)),
                (vc, Ast::Lit(L::B)),
                (vb, Ast::Lit(L::C)),
                (va, Ast::Lit(L::D)),
            ]),
            Ok(Ast::Lit(lit))
        );
    }

    for (var, matches) in vec![(va, false), (vb, true), (vc, false), (vd, false)] {
        assert_eq!(
            Ast::Var(var).sub(&sub![(vb, Ast::Lit(L::A))]),
            Ok(if matches {
                Ast::Lit(L::A)
            } else {
                Ast::Var(var)
            })
        )
    }

    assert_eq!(
        Ast::List(va.into(), vec![vb.into(), vc.into(), vd.into()]).sub(&sub![
            (va, Ast::Lit(L::A)),
            (vb, Ast::Lit(L::B)),
            (vc, Ast::Lit(L::C)),
            (vd, Ast::Lit(L::D)),
        ]),
        Ok(Ast::List(
            Term::Lit(L::A),
            vec![Ast::Lit(L::B), Ast::Lit(L::C), Ast::Lit(L::D)]
        )),
    );

    assert_eq!(
        Ast::Dict(
            va.into(),
            dict![(L::A, vb.into()), (L::B, vc.into()), (L::C, vd.into())]
        )
        .sub(&sub![
            (va, Ast::Lit(L::A)),
            (vb, Ast::Lit(L::B)),
            (vc, Ast::Lit(L::C)),
            (vd, Ast::Lit(L::D)),
        ]),
        Ok(Ast::Dict(
            Term::Lit(L::A),
            dict![
                (L::A, Ast::Lit(L::B)),
                (L::B, Ast::Lit(L::C)),
                (L::C, Ast::Lit(L::D)),
            ]
        ))
    );
}

#[test]
fn sub_term() {
    let va = Var::User(V::A);
    let vb = Var::User(V::B);
    let vc = Var::User(V::C);
    let vd = Var::User(V::D);

    for lit in vec![L::A, L::B, L::C, L::D] {
        assert_eq!(
            Term::Lit(lit).sub(&sub![
                (vd, Ast::Lit(L::A)),
                (vc, Ast::Lit(L::B)),
                (vb, Ast::Lit(L::C)),
                (va, Ast::Lit(L::D)),
            ]),
            Ok(Term::Lit(lit))
        );
    }

    for (var, matches) in vec![(va, false), (vb, true), (vc, false), (vd, false)] {
        assert_eq!(
            Term::Var(var).sub(&sub![(vb, Ast::Lit(L::A))]),
            Ok(if matches {
                Term::Lit(L::A)
            } else {
                Term::Var(var)
            })
        )
    }

    assert_eq!(
        Term::Var(va).sub(&sub![(va, Ast::List(Term::Lit(L::A), vec![]))]),
        Err(Error::BadTermSub),
    );

    assert_eq!(
        Term::Var(va).sub(&sub![(va, Ast::Dict(Term::Lit(L::A), dict![]))]),
        Err(Error::BadTermSub),
    );
}

#[test]
fn unify_ast() {
    let va = Var::User(V::A);
    let vb = Var::User(V::B);
    let vc = Var::User(V::C);
    let vd = Var::User(V::D);

    assert_eq!(Ast::Lit(L::A).unify(&Ast::Lit(L::B)), Bottom);

    assert_eq!(Ast::Lit(L::A).unify(&Ast::Lit(L::A)), top());

    assert_eq!(
        Ast::Var(va).unify(&Ast::Lit(L::A)),
        uok![(va, Ast::Lit(L::A))]
    );

    assert_eq!(Ast::Var(va).unify(&Ast::Var(va)), top());

    assert_eq!(Ast::Var(va).unify(&Ast::Var(vb)), uok![(va, Ast::Var(vb))]);

    assert_eq!(
        Ast::List(
            Term::Var(va),
            vec![Ast::Lit(L::B), Ast::Var(vc), Ast::Lit(L::D)]
        )
        .unify(&Ast::List(
            Term::Lit(L::A),
            vec![Ast::Var(vb), Ast::Lit(L::C), Ast::Var(vd)]
        )),
        uok![
            (va, Ast::Lit(L::A)),
            (vb, Ast::Lit(L::B)),
            (vc, Ast::Lit(L::C)),
            (vd, Ast::Lit(L::D)),
        ]
    );

    assert_eq!(
        Ast::Dict(
            Term::Var(va),
            dict![
                (L::A, Ast::Lit(L::B)),
                (L::B, Ast::Var(vc)),
                (L::C, Ast::Lit(L::D)),
                (L::D, Ast::Var(va)),
            ]
        )
        .unify(&Ast::Dict(
            Term::Lit(L::A),
            dict![
                (L::A, Ast::Var(vb)),
                (L::B, Ast::Lit(L::C)),
                (L::C, Ast::Var(vd)),
                (L::D, Ast::Lit(L::A)),
            ]
        )),
        uok![
            (va, Ast::Lit(L::A)),
            (vb, Ast::Lit(L::B)),
            (vc, Ast::Lit(L::C)),
            (vd, Ast::Lit(L::D)),
        ]
    )
}

#[test]
fn unify_term() {
    let va = Var::User(V::A);
    let vb = Var::User(V::B);

    assert_eq!(Term::Lit(L::A).unify(&Term::Lit(L::B)), Bottom);

    assert_eq!(Term::Lit(L::A).unify(&Term::Lit(L::A)), top());

    assert_eq!(
        Term::Var(va).unify(&Term::Lit(L::A)),
        uok![(va, Ast::Lit(L::A))]
    );

    assert_eq!(Term::Var(va).unify(&Term::Var(va)), top());

    assert_eq!(
        Term::Var(va).unify(&Term::Var(vb)),
        uok![(va, Ast::Var(vb))]
    );
}

#[test]
fn freshvars_varsrc() {
    let src = VarSource::new();

    let f = <VarSource as FreshVars<Var>>::acquire;

    assert_eq!(f(&src), Var::Auto(0));
    assert_eq!(f(&src), Var::Auto(1));
    assert_eq!(f(&src), Var::Auto(2));
    assert_eq!(f(&src), Var::Auto(3));
    assert_eq!(f(&src), Var::Auto(4));
}
