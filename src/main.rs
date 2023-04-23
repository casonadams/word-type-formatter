mod formatter;
use formatter::format;
use tokio::io::{AsyncBufReadExt, BufReader};

async fn read_stdin() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut buffer = Vec::new();
    reader.read_until(255, &mut buffer).await?;
    Ok(buffer)
}

fn vec_to_string(data: Vec<u8>) -> String {
    String::from_utf8(data).unwrap()
}

#[tokio::main]
async fn main() {
    // let mut buffer = String::new();
    // let stdin = tokio::io::stdin();
    // stdin.read_line(&mut buffer)?;

    let buffer = read_stdin().await.unwrap();
    let input = vec_to_string(buffer);

    match format(&input) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    }
}
