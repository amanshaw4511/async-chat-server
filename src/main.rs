use std::{io::Error, net::SocketAddr};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("Localhost:8000").await?;
    let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(10);
    loop {
        let (mut socket, addr) = listener.accept().await?;
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let (sock_reader, mut sock_writer) = socket.split();
            let mut reader = BufReader::new(sock_reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    byte_read = reader.read_line(&mut line) => {
                        if byte_read.unwrap() == 0 {
                            break;
                        }
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();
                        if addr != other_addr {
                            sock_writer.write_all("\t >> ".as_bytes()).await.unwrap();
                            sock_writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }

                }
            }
        });
    }
    // Ok(())
}
