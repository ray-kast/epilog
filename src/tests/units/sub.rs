use crate::{dict, prelude::*, sub, sub::Sub, tests::mocks::sub::*};
use std::collections::HashMap;

#[test]
fn sub_sub() {
    assert_eq!(Sub::top().with("a", Lit("A")), Ok(sub![("a", Lit("A"))]));

    assert_eq!(
        sub![("a", Var("b"))].with("b", Lit("B")),
        Ok(sub![("a", Lit("B")), ("b", Lit("B"))]),
    );

    assert_eq!(
        sub![("a", Lit("A"))].with("a", Lit("B")),
        Ok(sub![("a", Lit("B"))]),
    );
}

#[test]
fn sub_option() {
    assert_eq!(None::<Val>.with("a", Lit("A")), Ok(None));

    assert_eq!(Some(Var("a")).with("a", Lit("B")), Ok(Some(Lit("B"))));
}

#[test]
fn sub_vec() {
    assert_eq!(Vec::<Val>::new().with("a", Lit("A")), Ok(vec![]));

    assert_eq!(vec![Var("a")].with("a", Lit("A")), Ok(vec![Lit("A")]));

    assert_eq!(
        vec![Var("a"), Lit("a"), Var("b"), Lit("b")].sub(&sub![("a", Lit("A")), ("b", Lit("B"))]),
        Ok(vec![Lit("A"), Lit("a"), Lit("B"), Lit("b")]),
    );
}

#[test]
fn sub_hashmap() {
    assert_eq!(
        HashMap::<&'static str, Val>::new().with("a", Lit("A")),
        Ok(dict![]),
    );

    assert_eq!(
        dict![("a", Var("a"))].with("a", Lit("A")),
        Ok(dict![("a", Lit("A"))]),
    );

    assert_eq!(
        dict![
            ("a", Var("a")),
            ("b", Lit("a")),
            ("c", Var("b")),
            ("d", Lit("b"))
        ]
        .sub(&sub![("a", Lit("A")), ("b", Lit("B"))]),
        Ok(dict![
            ("a", Lit("A")),
            ("b", Lit("a")),
            ("c", Lit("B")),
            ("d", Lit("b")),
        ])
    );
}
