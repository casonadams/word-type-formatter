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

pub fn format(input: &str) -> String {
    let words: Vec<&str> = input.split(' ').collect();
    let mut output: Vec<String> = vec![];

    for word in words {
        let clean_word = word.trim();
        let formatter: Box<dyn Formatter> = match clean_word {
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

        output.push(formatter.format(clean_word));
    }
    output.join(" ")
}

#[cfg(test)]
mod tests {
    use super::format;
    use rstest::rstest;

    #[rstest]
    #[case("apple banana mango\n", "APPLE BANANA MANGO")]
    #[case("carrot zucchini broccoli\n", "[carrot] [zucchini] [broccoli]")]
    #[case(
        "horse giraffe mouse pigeon\n",
        "h*o*r*s*e g*i*r*a*f*f*e m*o*u*s*e p*i*g*e*o*n"
    )]
    #[case("box chair\n", "Unknown word: box Unknown word: chair")]
    fn test_format(#[case] input: &str, #[case] expected: &str) {
        let actual = format(input);
        assert_eq!(expected, &actual);
    }
}
