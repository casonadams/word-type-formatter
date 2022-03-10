mod formatter;
use formatter::format;

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer)?;

    println!("{}", format(&buffer));
    Ok(())
}
