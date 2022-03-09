use super::Formatter;

pub struct UnknownFormatter;
impl Formatter for UnknownFormatter {
    fn format(&self, s: &str) -> String {
        format!("Unknown word: {}", s)
    }
}
