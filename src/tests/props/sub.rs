use crate::{prelude::*, sub::CowSub, tests::strats::sub::*};
use proptest::prelude::*;

proptest! {
    #[test]
    fn nullsub_sub(sub in sub(30)) {
        prop_assert_eq!(sub.sub(&Sub::top()), Ok(sub));
    }

    #[test]
    fn nullsub_option(opt in prop::option::weighted(0.85, val(30))) {
        prop_assert_eq!(opt.sub(&Sub::top()), Ok(opt));
    }

    #[test]
    fn nullsub_vec(vec in prop::collection::vec(val(30), 0..30)) {
        prop_assert_eq!(vec.sub(&Sub::top()), Ok(vec));
    }

    #[test]
    fn nullsub_hashmap(map in prop::collection::hash_map(0..30, val(30), 0..30)) {
        prop_assert_eq!(map.sub(&Sub::top()), Ok(map));
    }

    #[test]
    fn sub_cowsub(sub in sub(30), sub2 in sub(30)) {
        prop_assert_eq!(CowSub::Owned(sub.clone()).sub(&sub2), sub.sub(&sub2).map(CowSub::Owned));
    }
}
