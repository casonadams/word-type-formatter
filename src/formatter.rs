pub mod animal_formatter;
pub mod fruit_formatter;
pub mod unknown_formatter;
pub mod vegetable_formatter;

pub use animal_formatter::AnimalFormatter;
pub use fruit_formatter::FruitFormatter;
pub use unknown_formatter::UnknownFormatter;
pub use vegetable_formatter::VegetableFormatter;

pub trait Formatter {
    fn format(&self, s: &str) -> String;
}

pub fn format_word(word: &str) -> String {
    let formatter: Box<dyn Formatter> = match word {
        "apple" => Box::new(FruitFormatter),
        "banana" => Box::new(FruitFormatter),
        "mango" => Box::new(FruitFormatter),
        "carrot" => Box::new(VegetableFormatter),
        "zucchini" => Box::new(VegetableFormatter),
        "broccoli" => Box::new(VegetableFormatter),
        "horse" => Box::new(AnimalFormatter),
        "giraffe" => Box::new(AnimalFormatter),
        "mouse" => Box::new(AnimalFormatter),
        "pigeon" => Box::new(AnimalFormatter),
        _ => Box::new(UnknownFormatter),
    };

    formatter.format(word)
}

#[cfg(test)]
mod tests {
    use super::format_word;
    use rstest::rstest;

    #[rstest]
    #[case("apple", "APPLE")]
    #[case("banana", "BANANA")]
    #[case("mango", "MANGO")]
    #[case("carrot", "[carrot]")]
    #[case("zucchini", "[zucchini]")]
    #[case("broccoli", "[broccoli]")]
    #[case("horse", "h*o*r*s*e")]
    #[case("giraffe", "g*i*r*a*f*f*e")]
    #[case("mouse", "m*o*u*s*e")]
    #[case("pigeon", "p*i*g*e*o*n")]
    #[case("chair", "Unknown word: chair")]
    fn test_format_word(#[case] input: &str, #[case] expected: &str) {
        let actual = format_word(input);
        assert_eq!(expected, &actual);
    }
}
