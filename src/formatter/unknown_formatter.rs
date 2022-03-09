use super::Formatter;

pub struct UnknownFormatter;
impl Formatter for UnknownFormatter {
    fn format(&self, s: &str) -> String {
        format!("Unknown word: {}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("chair", "Unknown word: chair")]
    #[case("", "Unknown word: ")]
    fn test_format(#[case] input: &str, #[case] expected: &str) {
        let actual = UnknownFormatter::format(&UnknownFormatter, input);
        assert_eq!(expected, &actual);
    }
}
