mod formatter;
use formatter::format;

fn main() -> Result<(), std::io::Error> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer)?;

    match format(&buffer) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    }
    Ok(())
}
