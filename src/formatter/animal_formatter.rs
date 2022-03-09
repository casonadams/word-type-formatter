use super::Formatter;

pub struct AnimalFormatter;
impl Formatter for AnimalFormatter {
    fn format(&self, s: &str) -> String {
        let mut word: String = String::from("");
        for (i, l) in s.chars().enumerate() {
            word.push(l);
            if i < s.len() - 1 {
                word.push('*')
            }
        }
        word
    }
}
