use super::Formatter;

pub struct FruitFormatter;
impl Formatter for FruitFormatter {
    fn format(&self, s: &str) -> String {
        s.to_uppercase()
    }
}
