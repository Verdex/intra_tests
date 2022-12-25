

#[cfg(test)]
mod test {
    use intra::*;

    #[test]
    fn execute_statements_should_execute() {
        let x = 0;

        let mut items = vec![];
        atom!(x => { items.push(1); }; { items.push(2); } => { items.push(3); });

        assert_eq!(items, [1, 2, 3]);
    }
}