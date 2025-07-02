use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod parser;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    loop {
        let (mut stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 512];
            loop {
                match stream.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        let mut parser = parser::Resp::new(&buf[..n]);
                        let resp = match parser.parse_texts() {
                            Ok(r) => r,
                            Err(e) => {
                                eprintln!("Parse error: {}", e);
                                continue;
                            }
                        };
                        if resp.is_empty() {
                            continue;
                        }
                        match resp[0].as_str() {
                            "PING" => {
                                if let Err(e) = stream.write_all(b"+PONG\r\n").await {
                                    eprintln!("Write error: {}", e);
                                    break;
                                }
                            }
                            "ECHO" => {
                                if resp.len() < 2 {
                                    eprintln!("ECHO missing argument");
                                    continue;
                                }
                                let msg = &resp[1];
                                let response = format!("+{}\r\n", msg);
                                if let Err(e) = stream.write_all(response.as_bytes()).await {
                                    eprintln!("Write error: {}", e);
                                    break;
                                }
                            }
                            _ => {
                                let err_msg = format!("-ERR unknown command '{}'\r\n", resp[0]);
                                if let Err(e) = stream.write_all(err_msg.as_bytes()).await {
                                    eprintln!("Write error: {}", e);
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Read error: {}", e);
                        break;
                    }
                }
            }
        });
    }
}
