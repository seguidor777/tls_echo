use futures::StreamExt;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_openssl::SslStream;

async fn handle_client(inbound: &mut SslStream<TcpStream>) {
    println!("Connection accepted");
    let mut buf = [0; 1024];

    loop {
        let n = inbound
            .read(&mut buf)
            .await
            .expect("failed to read data from inbound stream");

        if n == 0 {
            return;
        }

        inbound
            .write_all(&buf[0..n])
            .await
            .expect("failed to write data to inbound stream");
    }
}

#[tokio::main]
async fn main() {
    let mut acceptor = SslAcceptor::mozilla_modern(SslMethod::tls()).unwrap();

    acceptor
        .set_private_key_file("./ssl/key.pem", SslFiletype::PEM)
        .unwrap();
    acceptor
        .set_certificate_chain_file("./ssl/certs.pem")
        .unwrap();
    acceptor.check_private_key().unwrap();

    let acceptor = Arc::new(acceptor.build());
    let mut listener = TcpListener::bind("0.0.0.0:8443")
        .await
        .expect("cannot start listener");
    let mut incoming = listener.incoming();

    while let Some(connection) = incoming.next().await {
        match connection {
            Ok(connection) => {
                match tokio_openssl::accept(&acceptor, connection).await {
                    Ok(mut stream) => {
                        handle_client(&mut stream).await;
                    }
                    Err(_) => { /* connection failed */ }
                }
            }
            Err(_) => { /* connection failed */ }
        }
    }
}
