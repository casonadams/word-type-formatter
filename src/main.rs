mod formatter;
use argh::FromArgs;
use formatter::format_word;

#[derive(FromArgs)]
/// word-type-formatter
struct GoUp {
    /// input string
    #[argh(option, short = 'i')]
    input: String,
}

fn main() {
    let up: GoUp = argh::from_env();
    println!("{} => {}", &up.input, format_word(&up.input));
}
