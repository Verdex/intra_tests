
static TEST : u8 = 0xFF;

#[cfg(test)]
mod test {
    use intra::*;

    #[test]
    fn execute_statements_should_execute() {
        let _x = 0;

        let mut items = vec![];
        atom!(_x => { items.push(1); }; { items.push(2); } => { items.push(3); });

        assert_eq!(items, [1, 2, 3]);
    }

    #[test]
    fn execute_statements_should_be_visible_to_resolution() {
        let _x = 0;

        let mut ret = 0;
        atom!(_x => { let y = 1; }; { let z = 2; } => { ret = z + y; });

        assert_eq!(ret, 3);
    }

    #[test]
    fn execute_statements_should_be_visible_to_subsequent_execute_statements() {
        let _x = 0;

        let mut ret = 0;
        atom!(_x => { let y = 1; }; { let z = y + 1; } => { ret = z + y; });

        assert_eq!(ret, 3);
    }

    #[test]
    fn pattern_statements_should_pattern_match() {
        let x = 1;

        let mut ret = 0;
        atom!(x => [ y ] => { ret = y; });

        assert_eq!(ret, 1);
    }

    #[test]
    fn pattern_statements_should_pattern_match_with_condition() {
        let x = 1;

        let mut ret = 0;
        atom!(x => [ y if y == 1 ] => { ret = y; });

        assert_eq!(ret, 1);
    }

    #[test]
    fn initial_statement_should_work_with_namespace() {
        let mut ret = 0;
        atom!(crate::atom_tests::TEST => [ y if y != 0 ] => { ret = y; });

        assert_eq!(ret, crate::atom_tests::TEST);
    }

}