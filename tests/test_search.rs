
#[cfg(test)]

mod tests {

    use rust_properties::search;
    use rust_properties::search_case_insensitive;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust=Trust";

        assert_eq!(
            vec!["Trust"],
            search_case_insensitive(query, contents)
        );
    }
}
