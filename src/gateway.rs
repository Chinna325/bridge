// use std::clone;

use crate::{Context, protos::request, protos::response::Response};
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
use crate::traits::RequestHandler;
impl request::Request {
    pub async fn handle(&self, backend: &TWServer, ctx: &mut Context) -> Result<Response, ()> {
        match &self.operation {
            Some(request::request::Operation::AddUser(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::VerifyUser(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::RemoveUser(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::AddPost(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ChangePassword(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::GetProfilePicture(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::GetUser(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::UpdateUser(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::RemoveProfilePicture(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::RemoveReply(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::RemovePost(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::GetPost(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::SignIn(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::SignOut(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::Follow(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::UnFollow(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ListFollowers(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ListReplies(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::RepostPost(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ListPosts(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ReplyToPost(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ReactToPost(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::UndoReactToPost(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::UpdatePost(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::EditReply(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::GetReply(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::UpdateProfilePicture(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ResetPassword(req)) => {
                return req.handle(backend, ctx).await;
            }

            Some(request::request::Operation::CreateOneToOneConversation(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::CreateGroup(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::UpdateGroup(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ListGroups(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::AddUserToGroup(req)) => {
                return req.handle(backend, ctx).await;
            }

            Some(request::request::Operation::RemoveUserFromGroup(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ExitFromGroup(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::GetConversation(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ClearConversation(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::SendMessage(req)) => {
                return req.handle(backend, ctx).await;
            }

            Some(request::request::Operation::RemoveMessage(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::EditMessage(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ListConversation(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ReadMessage(req)) => {
                return req.handle(backend, ctx).await;
            }

            Some(request::request::Operation::AddAttachment(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ListAttachements(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::RemoveAttachement(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ReadAttchment(req)) => {
                return req.handle(backend, ctx).await;
            }
            _ => return Err(()),
        }
    }
}
