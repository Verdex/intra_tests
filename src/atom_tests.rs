
static TEST : u8 = 0xFF;

fn plus_2(input : u8) -> u8 { input + 2 }

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
    #[allow(unused_assignments)]
    fn execute_statements_should_be_visible_to_resolution() {
        let _x = 0;

        let mut ret = 0;
        atom!(_x => { let y = 1; }; { let z = 2; } => { ret = z + y; });

        assert_eq!(ret, 3);
    }

    #[test]
    #[allow(unused_assignments)]
    fn execute_statements_should_be_visible_to_subsequent_execute_statements() {
        let _x = 0;

        let mut ret = 0;
        atom!(_x => { let y = 1; }; { let z = y + 1; } => { ret = z + y; });

        assert_eq!(ret, 3);
    }

    #[test]
    fn execute_statement_after_last_pattern_should_be_visible() {
        let x = 0;

        let mut ret = 0;
        atom!(x => [ 0 ]; { let z = 5; } => { ret = z });

        assert_eq!(ret, 5);
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

    #[test]
    fn pre_map_statement_should_map() {
        fn plus_1(input : u8) -> u8 { input + 1 }

        let x = 1;

        let mut ret = 0;
        atom!(x => plus_1 $ [ y ] => { ret = y; });

        assert_eq!(ret, 2);
    }

    #[test]
    fn pre_map_statement_should_map_from_other_namespace() {
        let x = 1;

        let mut ret = 0;
        atom!(x => super::plus_2 $ [ y ] => { ret = y; });

        assert_eq!(ret, 3);
    }

    #[test]
    fn next_statement_should_pass_through_single_next() {
        let x = (1, 2);

        let mut ret = vec![];
        atom!(x => [ (a, _) ] a; [ x ] => { ret.push(x); });

        assert_eq!(ret, [1]);
    }

    #[test]
    fn next_statement_should_pass_through_multiple_nexts() {
        let x = (1, 2);

        let mut ret = vec![];
        atom!(x => [ (a, b) ] a, b; [ x ] => { ret.push(x); });

        assert_eq!(ret, [1, 2]);
    }

    #[test]
    fn next_statement_should_pass_through_nested_nexts() {
        let x = ((1, 2), (3, 4));

        let mut ret = vec![];
        atom!(x => [ (a, b) ] a, b; [ (c, d) ] c, d; [ x ] => { ret.push(x); });

        assert_eq!(ret, [1, 2, 3, 4]);
    }

    #[test]
    fn next_statement_should_pass_through_nested_nexts_with_filter() {
        let x = ((1, 2), (3, 4));

        let mut ret = vec![];
        atom!(x => [ (a, b) ] a, b; [ (c, d) ] c, d; [ x if x % 2 == 0 ] => { ret.push(x); });

        assert_eq!(ret, [2, 4]);
    }

    #[test]
    fn should_be_able_to_pattern_match_vector_in_pattern() {
        fn slice<'a>(input : &'a Vec<u8>) -> &'a [u8] { &input[..] }

        let x = Some(vec![1, 2]);

        let mut ret = vec![];
        atom!(x => [ Some(ref v) ] v; slice $ [ [a, b] ] a, b; [ x ] => { ret.push(*x); });

        assert_eq!(ret, [1, 2]);
    }
}