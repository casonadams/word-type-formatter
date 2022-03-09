use super::Formatter;

pub struct VegetableFormatter;
impl Formatter for VegetableFormatter {
    fn format(&self, s: &str) -> String {
        format!("[{}]", s)
    }
}
