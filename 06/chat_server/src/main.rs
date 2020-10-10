use std::sync::Arc;

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> io::Result<()> {
    let address = "127.0.0.1:5000";
    let mut listener = TcpListener::bind(address).await?;

    println!("Server is listening on {}", address);

    let (tx, _) = broadcast::channel(16);
    let tx = Arc::new(tx);

    loop {
        let (stream, client_address) = listener.accept().await?;
        println!("New client: {:?}", client_address);

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        let (rd, mut wr) = io::split(stream);
        let mut rd = BufReader::new(rd);

        tokio::spawn(async move {
            let mut buf = String::new();

            loop {
                buf.clear();
                match rd.read_line(&mut buf).await {
                    Ok(0) => {
                        println!("Client has closed the connection.");
                        return;
                    }
                    Ok(_) => {
                        println!("Server received: {}", buf);
                        tx.send(buf.clone()).unwrap();
                    }
                    Err(e) => {
                        eprintln!("Server failed to read from socket; err = {:?}", e);
                        return;
                    }
                }
            }
        });

        tokio::spawn(async move {
            loop {
                let message = rx.recv().await.unwrap();
                println!("Sending message to client: {}", message);
                wr.write_all(message.as_bytes()).await.unwrap();
                wr.flush().await.unwrap();
            }
        });
    }
}
