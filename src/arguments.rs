use std::iter;

pub struct Arguments(Vec<String>);

impl Arguments {
    pub fn as_slice(&self) -> &[String] {
        &self.0
    }
}

impl From<&str> for Arguments {
    fn from(value: &str) -> Self {
        Self(parse(value))
    }
}

impl std::fmt::Display for Arguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join(" "))
    }
}

fn parse(arguments: &str) -> Vec<String> {
    arguments
        .replace("''", "")
        .split('\'')
        .enumerate()
        .flat_map(|(i, part)| {
            if i.is_multiple_of(2) {
                Box::new(part.split_whitespace()) as Box<dyn Iterator<Item = &str>>
            } else {
                Box::new(iter::once(part))
            }
        })
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn it_returns_an_empty_vec_on_emtpy_input() {
        let result = parse("");

        assert!(result.is_empty());
    }

    #[test]
    fn it_returns_a_single_argument_as_is() {
        let result = parse("foo");
        assert_eq!(result, vec!["foo"]);
    }

    #[test]
    fn it_trims_whitespace_around_a_single_argument() {
        let result = parse("   foo    ");
        assert_eq!(result, vec!["foo"]);
    }

    #[test]
    fn it_returns_multiple_arguments_separated_by_whitespace() {
        let result = parse("    foo     bar     ");
        assert_eq!(result, vec!["foo", "bar"]);
    }

    #[test]
    fn it_returns_a_single_quoted_input_without_the_quotes() {
        let result = parse("'foo'");
        assert_eq!(result, vec!["foo"]);
    }

    #[test]
    fn it_removes_whitespace_around_quoted_input() {
        let result = parse("   'foo'    ");
        assert_eq!(result, vec!["foo"]);
    }

    #[test]
    fn it_keeps_whitespace_within_quotes() {
        let result = parse("'foo   bar'");
        assert_eq!(result, vec!["foo   bar"])
    }

    #[test]
    fn it_handles_quotes_after_an_unquoted_element() {
        let result = parse("foo  bar    'bar   baz'");
        assert_eq!(result, vec!["foo", "bar", "bar   baz"]);
    }

    #[test]
    fn it_concatenates_adjacent_quoted_elements() {
        let result = parse("'foo''bar'");
        assert_eq!(result, vec!["foobar"]);
    }

    #[test]
    fn it_removes_emtpy_quotes() {
        let result = parse("foo''bar");
        assert_eq!(result, vec!["foobar"]);
    }
}
