// #![deny(unused)]
mod fmt;
mod gateway;
mod protos;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
pub mod operations;
mod ws;
use jbackend_runtime::TWServer;
use native_tls::Identity;
use std::fs;
mod crypto;
use std::sync::OnceLock;

pub struct Context {
    pub email: String,
    pub is_acuthenticated: bool,
    pub user_name: String,
}

impl Context {
    pub fn new() -> Self {
        Self {
            email: String::new(),
            is_acuthenticated: false,
            user_name: String::new(),
        }
    }
}

static BACKEND: OnceLock<TWServer> = OnceLock::new();

#[tokio::main]
async fn main() {
    let cert = fs::read("./pem.crt").expect("Failed to read server certificate");
    let key = fs::read("./pem.key").expect("Failed to read server private key");
    let identity = Identity::from_pkcs8(&cert, &key).expect("Failed to add server config");
    let native_tls_acceptor = native_tls::TlsAcceptor::builder(identity).build().unwrap();
    let tls_acceptor = tokio_native_tls::TlsAcceptor::from(native_tls_acceptor);
    let listener = TcpListener::bind("0.0.0.0:6677").await.unwrap();

    let backend = TWServer::new().await.unwrap();

    let _ = BACKEND.get_or_init(|| backend);

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
        let backend = match BACKEND.get() {
            Some(backend) => backend,
            None => {
                continue;
            }
        };

        tokio::spawn(async move {
            match acceptor.accept(stream).await {
                Ok(tls_stream) => match accept_async(tls_stream).await {
                    Ok(ws_stream) => {
                        let mut client = ws::WsClient::new(ws_stream);
                        let mut ctx = Context::new();
                        match client.serve(backend, &mut ctx).await {
                            Ok(_) => {
                                let _ = client.close().await;
                            }
                            Err(_) => {
                                let _ = client.close().await;
                            }
                        }
                        println!("clinet is closed {:?}", addr);
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
