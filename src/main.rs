#![deny(unused)]
use std::vec;
pub mod backend;
pub mod cache;
pub mod errors;
pub mod process;
pub mod request;
pub mod response;
pub mod service_grpc;
pub mod service_request;
pub mod service_response;
pub mod tweet;
pub mod users;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use crate::cache::Cache;

pub struct Context {
    pub email: String,
    pub is_acuthenticated: bool,
    pub cache: Cache,
}

impl Context {
    pub fn new() -> Self {
        Self {
            email: String::new(),
            is_acuthenticated: false,
            cache: Cache::new(),
        }
    }
}

#[tokio::main]
async fn main() {
    let listner = TcpListener::bind("127.0.0.1:6677").await.unwrap();
    println!("Bridge server strt listening at 6677");
    loop {
        let (mut conn, addr) = listner.accept().await.unwrap();
        println!("Client is try to connect from {}", addr.ip());
        let mut buffer = vec![0; 1024];
        match conn.read(&mut buffer).await {
            Ok(size) => {
                let data = buffer[0..size].to_vec();
                let mut ctx = Context::new();
                let resp = process::process_request(data, &mut ctx)
                    .await
                    .unwrap_or_default();
                match conn.write(&resp).await {
                    Ok(_) => {}
                    Err(e) => {
                        println!("failed to send response :{:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("failed to read the data from the client {:?}", e);
            }
        }
    }
}
