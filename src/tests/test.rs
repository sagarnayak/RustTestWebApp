#[cfg(test)]
mod test_case_set_one {
    #[test]
    #[should_panic]
    fn test_one() {
        panic!("oh no!");
    }

    #[test]
    fn test_two() {
        assert_eq!(2, 1 + 1);
    }
}