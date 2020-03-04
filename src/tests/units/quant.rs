use crate::{quant::For, set, tests::mocks::ast::*};

#[test]
fn gen() {
    let va = Var::User(V::A);
    let vb = Var::User(V::B);
    let vc = Var::User(V::C);
    let vd = Var::User(V::D);

    let ast = Ast::List(Term::Var(va), vec![vb.into(), vc.into(), vd.into()]);
    assert_eq!(
        For::gen(ast.clone()),
        For::gen_on(set![va, vb, vc, vd], ast)
    );
}
