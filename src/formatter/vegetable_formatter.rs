use super::Formatter;

pub struct VegetableFormatter;
impl Formatter for VegetableFormatter {
    fn format(&self, s: &str) -> String {
        format!("[{}]", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("carrot", "[carrot]")]
    #[case("zucchini", "[zucchini]")]
    #[case("broccoli", "[broccoli]")]
    #[case("", "[]")]
    fn test_format(#[case] input: &str, #[case] expected: &str) {
        let actual = VegetableFormatter::format(&VegetableFormatter, input);
        assert_eq!(expected, &actual);
    }
}
