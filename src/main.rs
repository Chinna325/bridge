#![deny(unused)]
pub mod backend;
pub mod cache;
pub mod chat;
pub mod errors;
pub mod process;
pub mod request;
pub mod response;
pub mod service_grpc;
pub mod service_request;
pub mod service_response;
pub mod tweet;
pub mod users;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

use crate::{cache::Cache, process::WsClient};

pub struct Context {
    pub email: String,
    pub is_acuthenticated: bool,
    pub cache: Cache,
    pub user_name: String,
}

impl Context {
    pub fn new() -> Self {
        Self {
            email: String::new(),
            is_acuthenticated: false,
            cache: Cache::new(),
            user_name: String::new(),
        }
    }
}
use native_tls::Identity;
use std::fs;
#[tokio::main]
async fn main() {
    let cert = fs::read("./pem.crt").expect("Failed to read server certificate");
    let key = fs::read("./pem.key").expect("Failed to read server private key");
    let identity = Identity::from_pkcs8(&cert, &key).expect("Failed to add server config");
    let native_tls_acceptor = native_tls::TlsAcceptor::builder(identity).build().unwrap();
    let tls_acceptor = tokio_native_tls::TlsAcceptor::from(native_tls_acceptor);
    let listener = TcpListener::bind("0.0.0.0:6677").await.unwrap();
    println!("Web socket server start listening at 6677");
    loop {
        let res = listener.accept().await;
        let (stream, addr) = match res {
            Ok(val) => (val.0, val.1),
            Err(e) => {
                println!("failed to accept tcp connection {:?}", e);
                continue;
            }
        };
        println!("New connection from: {}", addr);
        let acceptor = tls_acceptor.clone();
        tokio::spawn(async move {
            // Perform TLS handshake
            match acceptor.accept(stream).await {
                Ok(tls_stream) => match accept_async(tls_stream).await {
                    Ok(ws_stream) => {
                        let mut client = WsClient::new(ws_stream);
                        let mut ctx = Context::new();
                        let _ = client.serve(&mut ctx).await;
                    }
                    Err(e) => {
                        eprintln!("Failed to accept web socket connections:{}", e);
                    }
                },
                Err(e) => println!("TLS handshake failed with {}: {}", addr, e),
            }
        });
    }
}
