use crate::{prelude::*, quant::For, tests::strats::ast::*};
use proptest::prelude::*;

proptest! {
    #[test]
    fn gen(
        ast in ast_safe(8, 30, 10),
    ) {
        prop_assert_eq!(
            For::gen(ast.clone()),
            For::gen_on(ast.free_vars().into_iter().copied().collect(), ast)
        );
    }

    #[test]
    fn inst(
        // Using ast_safe to preserve the fresh invariant of VarSource
        ast in ast_safe(8, 30, 10),
    ) {
        let src = VarSource::new();

        let vars: Vec<_> = ast.free_vars().into_iter().copied().collect();
        let instanced = For::gen(ast).inst(&src);

        let instanced = match instanced {
            Ok(i) => i,
            Err(e) => {
                prop_assert!(false, "Instancing failed: {:?}", e);
                unreachable!()
            },
        };

        let newvars = instanced.free_vars();

        for var in vars {
            prop_assert!(!newvars.contains(&var));
        }
    }
}
