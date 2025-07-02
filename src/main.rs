use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (mut stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 512];

            loop {
                match stream.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(_) => {
                        if let Err(e) = stream.write_all(b"+PONG\r\n").await {
                            eprintln!("Write error: {}", e);
                            break;
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

