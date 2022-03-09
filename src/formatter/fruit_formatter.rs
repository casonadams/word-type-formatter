use super::Formatter;

pub struct FruitFormatter;
impl Formatter for FruitFormatter {
    fn format(&self, s: &str) -> String {
        s.to_uppercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("apple", "APPLE")]
    #[case("banana", "BANANA")]
    #[case("mango", "MANGO")]
    #[case("", "")]
    fn test_format(#[case] input: &str, #[case] expected: &str) {
        let actual = FruitFormatter::format(&FruitFormatter, input);
        assert_eq!(expected, &actual);
    }
}
