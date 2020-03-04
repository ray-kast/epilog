use crate::{prelude::*, tests::strats::ast::*};
use proptest::prelude::*;

proptest! {
    #[test]
    fn nullsub_ast(ast in ast(8, 30, 10)) {
        prop_assert_eq!(ast.sub(&Sub::top()), Ok(ast));
    }

    #[test]
    fn sub_ast_list(
        term in term(30),
        vec in prop::collection::vec(ast(8, 30, 10), 0..30),
        sub in sub(8, 30, 10),
    ) {
        prop_assert_eq!(
            Ast::List(term, vec.clone()).sub(&sub),
            term.sub(&sub)
                .and_then(|t| vec.sub(&sub).map(|v| Ast::List(t, v)))
        );
    }

    #[test]
    fn sub_ast_dict(
        term in term(30),
        map in prop::collection::hash_map((0..30u32).prop_map(L), ast(8, 30, 10), 0..30),
        sub in sub(8, 30, 10),
    ) {
        prop_assert_eq!(
            Ast::Dict(term, map.clone()).sub(&sub),
            term.sub(&sub)
                .and_then(|t| map.sub(&sub).map(|d| Ast::Dict(t, d)))
        )
    }

    #[test]
    fn unify_ast_list(
        terml in term(30),
        termr in term(30),
        vecl in prop::collection::vec(ast(8, 30, 10), 0..30),
        vecr in prop::collection::vec(ast(8, 30, 10), 0..30),
    ) {
        let l = Ast::List(terml.clone(), vecl.clone());
        let r = Ast::List(termr.clone(), vecr.clone());

        prop_assert_eq!(l.unify(&r), terml.unify(&termr).and_then(|s| vecl.unify_with(s, &vecr)));
    }

    #[test]
    fn unify_ast_dict(
        terml in term(30),
        termr in term(30),
        mapl in prop::collection::hash_map((0..30u32).prop_map(L), ast(8, 30, 10), 0..30),
        mapr in prop::collection::hash_map((0..30u32).prop_map(L), ast(8, 30, 10), 0..30),
    ) {
        let l = Ast::Dict(terml.clone(), mapl.clone());
        let r = Ast::Dict(termr.clone(), mapr.clone());

        prop_assert_eq!(l.unify(&r), terml.unify(&termr).and_then(|s| mapl.unify_with(s, &mapr)));
    }
}
