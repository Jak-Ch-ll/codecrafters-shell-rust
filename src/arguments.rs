mod check_for_balanced_quotes;
mod take_while_peek;

pub use check_for_balanced_quotes::check_for_balanced_quotes;

use self::take_while_peek::TakeWhilePeekExtension;

pub struct Arguments(Vec<String>);

impl Arguments {
    pub fn as_slice(&self) -> &[String] {
        &self.0
    }
}

impl From<&str> for Arguments {
    fn from(input: &str) -> Self {
        let mut arguments: Vec<Vec<String>> = vec![vec![]];
        let mut iter = input.chars().peekable();

        let special_characters = ['\'', '"'];

        loop {
            let current = iter
                .take_while_peek(|c| !special_characters.contains(c) && !c.is_whitespace())
                .collect();

            arguments.last_mut().unwrap().push(current);

            let Some(next) = iter.next() else {
                // end of input
                break;
            };

            match next {
                quote_char @ ('\'' | '"') => {
                    let quoted = iter.by_ref().take_while(|c| *c != quote_char).collect();
                    arguments.last_mut().unwrap().push(quoted);
                }

                char if char.is_whitespace() => {
                    arguments.push(Vec::new());

                    iter.take_while_peek(|c| c.is_whitespace()).for_each(|_| {});
                }

                _ => unreachable!("`take_while_peek` should ensure we never reach this"),
            }
        }

        Self(
            arguments
                .iter()
                .map(|inner| inner.join(""))
                .filter(|s| !s.is_empty())
                .collect(),
        )
    }
}

impl std::fmt::Display for Arguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join(" ").replace(" \n ", "\n"))
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn it_returns_an_empty_vec_on_emtpy_input() {
        let result = Arguments::from("");

        assert!(
            result.as_slice().is_empty(),
            "Expected an empty vector, got: {:?}",
            result.as_slice()
        );
    }

    #[test]
    fn it_returns_a_single_argument_as_is() {
        let input = "foo";
        assert_eq!(
            Arguments::from(input).as_slice(),
            vec!["foo"],
            "Input: {input}"
        );
    }

    #[test]
    fn it_trims_whitespace_around_a_single_argument() {
        let input = "   foo    ";
        assert_eq!(
            Arguments::from(input).as_slice(),
            vec!["foo"],
            "Input: {input}"
        );
    }

    #[test]
    fn it_returns_multiple_arguments_separated_by_whitespace() {
        let input = "    foo     bar     ";
        assert_eq!(
            Arguments::from(input).as_slice(),
            vec!["foo", "bar"],
            "Input: {input}"
        );
    }

    #[test]
    fn it_returns_a_single_quoted_input_without_the_quotes() {
        let input = "'foo'";
        assert_eq!(
            Arguments::from(input).as_slice(),
            vec!["foo"],
            "Input: {input}"
        );

        let input = r#""foo""#;
        assert_eq!(
            Arguments::from(input).as_slice(),
            vec!["foo"],
            "Input: {input}"
        );
    }

    #[test]
    fn it_removes_whitespace_around_quoted_input() {
        let input = "   'foo'    ";
        assert_eq!(
            Arguments::from(input).as_slice(),
            vec!["foo"],
            "Input: {input}"
        );

        let input = r#"   "foo"    "#;
        assert_eq!(
            Arguments::from(input).as_slice(),
            vec!["foo"],
            "Input: {input}"
        );
    }

    #[test]
    fn it_keeps_whitespace_within_quotes() {
        let input = "'foo   bar'";
        assert_eq!(
            Arguments::from(input).as_slice(),
            vec!["foo   bar"],
            "Input: {input}"
        );

        let input = r#""foo   bar""#;
        assert_eq!(
            Arguments::from(input).as_slice(),
            vec!["foo   bar"],
            "Input: {input}"
        );
    }

    #[test]
    fn it_handles_quotes_after_an_unquoted_element() {
        let input = "foo  bar    'bar   baz'";
        assert_eq!(
            Arguments::from(input).as_slice(),
            vec!["foo", "bar", "bar   baz"],
            "Input: {input}"
        );

        let input = r#"foo  bar    "bar   baz""#;
        assert_eq!(
            Arguments::from(input).as_slice(),
            vec!["foo", "bar", "bar   baz"],
            "Input: {input}"
        );
    }

    #[test]
    fn it_concatenates_adjacent_quoted_elements() {
        let input = "'foo''bar'";
        assert_eq!(
            Arguments::from(input).as_slice(),
            vec!["foobar"],
            "Input: {input}"
        );

        let input = r#""foo""bar""#;
        assert_eq!(
            Arguments::from(input).as_slice(),
            vec!["foobar"],
            "Input: {input}"
        );
    }

    #[test]
    fn it_removes_emtpy_quotes() {
        let input = "foo''bar";
        assert_eq!(
            Arguments::from(input).as_slice(),
            vec!["foobar"],
            "Input: {input}"
        );

        let input = r#"foo""bar"#;
        assert_eq!(
            Arguments::from(input).as_slice(),
            vec!["foobar"],
            "Input: {input}"
        );
    }

    #[test]
    fn it_keeps_unquoted_and_quoted_elements_together() {
        let input = "foo'bar'baz";
        assert_eq!(
            Arguments::from(input).as_slice(),
            vec!["foobarbaz"],
            "Input: {input}"
        );

        let input = r#"foo"bar"baz"#;
        assert_eq!(
            Arguments::from(input).as_slice(),
            vec!["foobarbaz"],
            "Input: {input}"
        );
    }

    #[test]
    fn it_handles_a_mix_of_quotes() {
        let input = r#"foo 'bar' "baz" 'qux' "quux""#;
        assert_eq!(
            Arguments::from(input).as_slice(),
            vec!["foo", "bar", "baz", "qux", "quux"],
            "Input: {input}"
        );
    }

    #[test]
    fn it_handles_double_quotes_within_single_quotes() {
        let input = r#"foo 'bar "baz" qux'"#;
        assert_eq!(
            Arguments::from(input).as_slice(),
            vec!["foo", r#"bar "baz" qux"#],
            "Input: {input}"
        );
    }
}
