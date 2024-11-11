use std::{
    error::Error,
    fs::File,
    io::{self, BufReader},
    net::SocketAddr,
    sync::Arc,
};

use rustls_pemfile::{certs, private_key};
use tokio::net::TcpListener;
use tokio_rustls::{
    rustls::{ClientConfig, RootCertStore, ServerConfig},
    TlsAcceptor,
};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = "127.0.0.1".parse().unwrap();
    let cert = certs(&mut BufReader::new(File::open("").unwrap()))
        .map(|f| f.unwrap())
        .collect();
    let key = private_key(&mut BufReader::new(File::open("").unwrap()))
        .unwrap()
        .unwrap();
    let root_store = RootCertStore::empty();
    let client_config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let server_config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert, key)?;
    let acceptor = TlsAcceptor::from(Arc::new(server_config));
    let listener = TcpListener::bind(addr).await.unwrap();
    loop {
        let (stream, peer_addr) = listener.accept().await.unwrap();
        let acceptor = acceptor.clone();
        let fut = async move {
            let mut stream = acceptor.accept(stream).await.unwrap();
            Ok(()) as io::Result<()>
        };
        tokio::spawn(async move {
            if let Err(e) = fut.await {
                eprintln!("accept error: {:?}", e);
            }
        });
    }
}
