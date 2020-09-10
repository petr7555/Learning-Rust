use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let address = "127.0.0.1:5000";
    let socket = TcpStream::connect(address).await?;
    println!("Client connected to the server on {}", address);
    let (mut rd, mut wr) = io::split(socket);

    tokio::spawn(async move {
        let mut buf = [0; 1024];
        loop {
            match rd.read(&mut buf).await {
                Ok(0) => {
                    println!("Server has closed the connection.");
                    return;
                }
                Ok(_) => {
                    let s = match std::str::from_utf8(&buf) {
                        Ok(v) => v,
                        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                    };
                    println!("Client received: {}", s);
                }
                Err(e) => {
                    eprintln!("Client failed to read from socket; err = {:?}", e);
                    return;
                }
            }
        }
    });


    loop {
        let mut buf = String::new();
        println!("Enter text ('quit' for exit):");
        std::io::stdin().read_line(&mut buf)?;
        if buf.trim() == "quit" {
            wr.shutdown();
            return Ok(());
        }

        wr.write_all(buf.as_bytes()).await?;
    }
}