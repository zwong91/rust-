use std::net::TcpStream;
use std::io::{ prelude::*, BufReader, Write };
use std::str;

fn use_server(server: &str, port: u16, content: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect((server, port))?;
    let _ = stream.write(content.as_bytes())?;

    let mut reader = BufReader::new(&stream);
    let mut buffer: Vec<u8> = Vec::new();
    reader.read_until(b'\n', &mut buffer)?;

    println!("recv from server: {} ", str::from_utf8(&buffer).unwrap());
    Ok(())
}

// 迭代一
fn main() -> std::io::Result<()> {
    use_server("127.0.0.1", 8080, "use server-foo download 127.0.0.1:8080")?;
    use_server("127.0.0.1", 8081, "use server-bar download 127.0.0.1:8081")?;

    Ok(())
}