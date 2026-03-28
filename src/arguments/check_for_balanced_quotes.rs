pub fn check_for_balanced_quotes(input: &str) -> bool {
    let mut chars = input.chars();

    loop {
        let Some(quote_char) = chars.find(|c| *c == '\'' || *c == '"') else {
            return true;
        };

        if chars.all(|c| c != quote_char) {
            return false;
        }
    }
}

#[cfg(test)]
mod test {
    use super::check_for_balanced_quotes;

    #[test]
    fn it_returns_true_for_empty_input() {
        let input = "";
        assert!(check_for_balanced_quotes(input), "Input: {input}");
    }

    #[test]
    fn it_returns_false_for_a_single_quote() {
        let input = "'";
        assert!(!check_for_balanced_quotes(input), "Input: {input}");

        let input = r#"""#;
        assert!(!check_for_balanced_quotes(input), "Input: {input}");
    }

    #[test]
    fn it_returns_true_for_input_without_quotes() {
        let input = "foo";
        assert!(check_for_balanced_quotes(input), "Input: {input}");
    }

    #[test]
    fn it_returns_true_for_two_quotes() {
        let input = "''";
        assert!(check_for_balanced_quotes(input), "Input: {input}");

        let input = r#""""#;
        assert!(check_for_balanced_quotes(input), "Input: {input}");
    }

    #[test]
    fn it_returns_false_for_three_quotes() {
        let input = "'''";
        assert!(!check_for_balanced_quotes(input), "Input: {input}");
    }

    #[test]
    fn it_returns_true_for_one_quote_within_quotes() {
        let input = r#"'"'"#;
        assert!(check_for_balanced_quotes(input), "Input: {input}");

        let input = r#""'""#;
        assert!(check_for_balanced_quotes(input), "Input: {input}");
    }

    #[test]
    fn it_handles_more_complex_input() {
        let input = r#"
            cmd --path "/var/log/app/"
            --filter 'type="event" and status="ok"'
            --payload "How's it going?"
        "#;
        assert!(check_for_balanced_quotes(input), "Input: {input}");

        let input = r#"
            cmd --notes "unclosed segment with 'nested single quotes
        "#;
        assert!(!check_for_balanced_quotes(input), "Input: {input}");
    }
}
