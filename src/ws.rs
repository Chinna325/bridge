use crate::{Context, protos::request};
use futures::{SinkExt, StreamExt};
use jbackend_runtime::TWServer;
use prost::Message;
pub struct WsClient {
    stream: tokio_tungstenite::WebSocketStream<tokio_native_tls::TlsStream<tokio::net::TcpStream>>,
}
impl WsClient {
    pub fn new(
        stream: tokio_tungstenite::WebSocketStream<
            tokio_native_tls::TlsStream<tokio::net::TcpStream>,
        >,
    ) -> Self {
        Self { stream }
    }

    pub async fn serve(&mut self, backend: &TWServer, ctx: &mut Context) -> Result<(), ()> {
        loop {
            let data = match self.stream.next().await {
                Some(Ok(data)) => data,
                Some(Err(_)) => return Err(()),
                None => {
                    println!("Client disconnected");
                    return Ok(());
                }
            };

            match data {
                tokio_tungstenite::tungstenite::Message::Text(_)
                | tokio_tungstenite::tungstenite::Message::Frame(_)
                | tokio_tungstenite::tungstenite::Message::Pong(_) => {
                    return Err(());
                }
                tokio_tungstenite::tungstenite::Message::Close(_) => return Ok(()),
                tokio_tungstenite::tungstenite::Message::Ping(_) => {
                    let msg = tokio_tungstenite::tungstenite::Message::Pong(vec![].into());
                    if self.stream.send(msg.into()).await.is_err() {
                        return Err(());
                    }
                }
                tokio_tungstenite::tungstenite::Message::Binary(_) => {}
            }
            let data = data.into_data().to_vec();

            let req = match request::Request::decode(data.as_slice()) {
                Ok(req) => req,
                Err(_) => return Err(()),
            };

            println!("Req {}", req);

            let resp = req.handle(backend, ctx).await?;

            println!("Resp {}", resp);

            if self.stream.send(resp.encode_to_vec().into()).await.is_err() {
                return Err(());
            }
        }
    }
    pub async fn close(&mut self) {
        let _ = self
            .stream
            .send(tokio_tungstenite::tungstenite::Message::Close(None).into())
            .await;
    }
}
