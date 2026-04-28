// use std::clone;

use crate::{
    Context, backend, errors, request,
    response::{self, Response},
    service_request::{self},
    service_response,
};
use futures::{SinkExt, StreamExt};
use prost::Message;
use sha2::Digest;
use uuid::Uuid;
use vortex_otp_lib::{OtpCharSet, generate_otp};

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

    pub async fn serve(&mut self, ctx: &mut Context) -> Result<(), ()> {
        loop {
            let data = match self.stream.next().await {
                Some(Ok(data)) => data,
                Some(Err(_)) => return Err(()),
                None => {
                    println!("Client disconnected");
                    return Ok(());
                }
            };

            let data = data.into_data().to_vec();

            let req = match request::Request::decode(data.as_slice()) {
                Ok(req) => req,
                Err(_) => return Err(()),
            };

            println!("Req {:?}", req);

            let resp = req.handle(ctx).await.ok_or(())?;

            println!("Resp {:?}", resp);

            if self.stream.send(resp.encode_to_vec().into()).await.is_err() {
                return Err(());
            }
        }
    }
}
impl request::Request {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        match &self.operation {
            Some(request::request::Operation::AddUser(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::VerifyUser(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::RemoveUser(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::AddTweet(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::ChangePassword(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::GetProfilePicture(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::GetUser(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::UpdateUser(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::RemoveProfilePicture(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::RemoveReply(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::RemoveTweet(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::GetTweet(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::SignIn(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::SignOut(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::Follow(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::UnFollow(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::ListFollowers(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::ListReplies(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::RepostTweet(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::ListTweets(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::ReplyToTweet(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::ReactToTweet(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::UndoReactToTweet(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::UpdateTweet(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::EditReply(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::GetReply(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::UpdateProfilePicture(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::ResetPassword(req)) => {
                return req.handle(ctx).await;
            }

            Some(request::request::Operation::CreateOneToOneChat(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::CreateGroup(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::UpdateGroup(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::ListGroups(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::AddUserToGroup(req)) => {
                return req.handle(ctx).await;
            }

            Some(request::request::Operation::RemoveUserFromGroup(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::ExitFromGroup(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::GetChat(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::ClearChat(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::SendMessage(req)) => {
                return req.handle(ctx).await;
            }

            Some(request::request::Operation::RemoveMessage(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::EditMessage(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::ListChat(req)) => {
                return req.handle(ctx).await;
            }
            Some(request::request::Operation::ReadMessage(req)) => {
                return req.handle(ctx).await;
            }
            _ => return None,
        }
    }
}

impl request::AddUser {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        let email = self.email.clone();
        let user_name = self.user_name.clone();
        let password = self.password.clone();
        if email.is_empty() || user_name.is_empty() || password.is_empty() {
            return Some(errors::form_response("AddUser", response::Status::BackendError).await);
        }
        let email_otp = match generate_otp(6, OtpCharSet::Numeric) {
            Ok(otp) => otp,
            Err(_) => {
                return Some(
                    errors::form_response("AddUser", response::Status::BackendError).await,
                );
            }
        };
        println!("Email otp :{}", email_otp);
        let mut hasher = sha2::Sha256::new();
        hasher.update(password);
        let password_hash = hasher.finalize().to_vec();
        let mut hasher = sha2::Sha256::new();
        hasher.update(self.email.clone());
        let key = hasher.finalize().to_vec();
        let redis_object = service_request::RedisObject {
            email: email.clone(),
            user_name: user_name.clone(),
            email_otp: Some("111111".to_string()),
            password: password_hash,
        };
        let data = redis_object.encode_to_vec();
        let req = service_request::ServiceRequest {
            operation: Some(service_request::service_request::Operation::AddUser(
                service_request::AddUser {
                    email,
                    user_name,
                    data,
                    key,
                },
            )),
        };
        let client = backend::ceate_grpc_connection().await;
        let resp = req.execute(client).await;
        if resp.is_none() {
            return Some(errors::form_response("AddUser", response::Status::BackendError).await);
        }
        let resp = resp.unwrap();
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::AddUser(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Some(
                    errors::form_response("AddUser", response::Status::BackendError).await,
                );
            }
        }
        ctx.email = self.email.clone();
        Some(response::Response {
            operation: Some(response::response::Operation::AddUser(response::AddUser {
                status: response::Status::Success as i32,
                message: None,
            })),
        })
    }
}

impl request::VerifyUser {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        let mut hasher = sha2::Sha256::new();
        hasher.update(self.email.clone());
        let key = hasher.finalize().to_vec();
        let data = ctx.cache.get(key.clone()).await;
        if data.is_none() {
            return Some(errors::form_response("VerifyUser", response::Status::BackendError).await);
        }
        let data = data.unwrap();
        let object = service_request::RedisObject::decode(data.as_slice());
        if object.is_err() {
            return Some(errors::form_response("VerifyUser", response::Status::BackendError).await);
        }
        let object = object.unwrap();
        if object.email != self.email.clone() {
            return Some(errors::form_response("VerifyUser", response::Status::BackendError).await);
        }
        // if object.email_otp.unwrap_or_default() != self.email_otp.clone() {
        //     return Some(errors::form_response("VerifyUser", response::Status::BackendError).await);
        // }
        let res = service_response::User::new(
            object.email.clone(),
            object.password.clone(),
            object.user_name.clone(),
        )
        .await;
        if res.is_some() {
            return Some(errors::form_response("VerifyUser", response::Status::BackendError).await);
        }
        ctx.is_acuthenticated = true;
        ctx.email = self.email.clone();
        ctx.user_name = object.user_name.clone();
        Some(response::Response {
            operation: Some(response::response::Operation::VerifyUser(
                response::VerifyUser {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::RemoveUser {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("RemoveUser", response::Status::BackendError).await);
        }
        let user = service_response::User::get(self.user_name.clone()).await;
        if user.is_none() {
            return Some(errors::form_response("RemoveUser", response::Status::BackendError).await);
        }
        let user = user.unwrap();
        let mut hasher = sha2::Sha256::new();
        hasher.update(self.password.clone());
        let hash = hasher.finalize().to_vec();
        if hash != user.password {
            return Some(errors::form_response("RemoveUser", response::Status::BackendError).await);
        }
        let resp = user.remove().await;
        if resp.is_none() {
            return Some(errors::form_response("RemoveUser", response::Status::BackendError).await);
        }
        Some(response::Response {
            operation: Some(response::response::Operation::RemoveUser(
                response::RemoveUser {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::ChangePassword {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("ChangePassword", response::Status::BackendError).await,
            );
        }
        if self.old_password.is_empty() || self.new_password.is_empty() {
            return Some(
                errors::form_response("ChangePassword", response::Status::BackendError).await,
            );
        }
        let user = service_response::User::get(self.user_name.clone()).await;
        if user.is_none() {
            return Some(
                errors::form_response("ChangePassword", response::Status::BackendError).await,
            );
        }
        let mut user = user.unwrap();
        let mut hasher = sha2::Sha256::new();
        hasher.update(self.old_password.clone());
        let hash = hasher.finalize().to_vec();
        if user.password != hash {
            return Some(
                errors::form_response("ChangePassword", response::Status::BackendError).await,
            );
        }
        let mut hasher = sha2::Sha256::new();
        hasher.update(self.new_password.clone());
        let hash = hasher.finalize().to_vec();
        user.password = hash;
        let res = user.update().await;
        if res.is_none() {
            return Some(
                errors::form_response("ChangePassword", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::ChangePassword(
                response::ChangePassword {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::GetProfilePicture {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("GetProfilePicture", response::Status::BackendError).await,
            );
        }
        let user = service_response::User::get(self.user_name.clone()).await;
        if user.is_none() {
            return Some(
                errors::form_response("GetProfilePicture", response::Status::BackendError).await,
            );
        }
        let user = user.unwrap();
        let data = user.get_profile_picture().await;
        if data.is_none() {
            return Some(
                errors::form_response("GetProfilePicture", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::GetProfilePicture(
                response::GetProfilePicture {
                    status: response::Status::Success as i32,
                    message: None,
                    data: data.unwrap(),
                },
            )),
        })
    }
}

impl request::GetUser {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("GetUser", response::Status::BackendError).await);
        }
        let user = service_response::User::get(self.user_name.clone()).await;
        if user.is_none() {
            return Some(errors::form_response("GetUser", response::Status::BackendError).await);
        }
        let user = user.unwrap();
        Some(response::Response {
            operation: Some(response::response::Operation::GetUser(response::GetUser {
                status: response::Status::Success as i32,
                message: None,
                user: Some(response::User {
                    email: user.email.clone(),
                    create_at: 0_u64,
                }),
            })),
        })
    }
}

impl request::UpdateUser {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("UpdateUser", response::Status::BackendError).await);
        }
        Some(response::Response {
            operation: Some(response::response::Operation::UpdateUser(
                response::UpdateUser {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::SignIn {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if self.user_name.is_some() {
            if self.password.is_none() {
                return Some(errors::form_response("SignIn", response::Status::BackendError).await);
            }
            let user_name = self.user_name.clone().unwrap();
            let user = service_response::User::get(user_name.clone()).await;
            if user.is_none() {
                println!("user is not found");
                return Some(errors::form_response("SignIn", response::Status::BackendError).await);
            }
            let user = user.unwrap();
            let passowrd = self.password.clone().unwrap();
            let mut hasher = sha2::Sha256::new();
            hasher.update(passowrd);
            let hash = hasher.finalize().to_vec();
            if user.password != hash {
                println!("password is not matched");
                return Some(errors::form_response("SignIn", response::Status::BackendError).await);
            }
            ctx.is_acuthenticated = true;
            ctx.email = user.email.clone();
            ctx.user_name = self.user_name.clone().unwrap();
        } else {
            return Some(response::Response {
                operation: Some(response::response::Operation::SignIn(response::SignIn {
                    status: response::Status::BackendError as i32,
                    message: Some("Invalid Credentials".to_string()),
                })),
            });
            //jwt token
        }
        Some(response::Response {
            operation: Some(response::response::Operation::SignIn(response::SignIn {
                status: response::Status::Success as i32,
                message: None,
            })),
        })
    }
}

impl request::SignOut {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("SignOut", response::Status::BackendError).await);
        }
        ctx.is_acuthenticated = false;
        ctx.user_name = String::new();
        ctx.email = String::new();
        Some(response::Response {
            operation: Some(response::response::Operation::SignOut(response::SignOut {
                status: response::Status::Success as i32,
                message: None,
            })),
        })
    }
}

impl request::Follow {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("Follow", response::Status::BackendError).await);
        }
        let user = service_response::User::get(ctx.user_name.clone()).await;
        if user.is_none() {
            return Some(errors::form_response("Follow", response::Status::BackendError).await);
        }
        let user = service_response::User::get(self.user_name.clone()).await;
        if user.is_none() {
            return Some(errors::form_response("Follow", response::Status::BackendError).await);
        }
        let user = user.unwrap();
        let resp = user.follow(self.user_name.clone()).await;
        if resp.is_none() {
            return Some(errors::form_response("Follow", response::Status::BackendError).await);
        }
        Some(response::Response {
            operation: Some(response::response::Operation::Follow(response::Follow {
                status: response::Status::Success as i32,
                message: None,
            })),
        })
    }
}

impl request::UnFollow {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("UnFollow", response::Status::BackendError).await);
        }
        let user = service_response::User::get(ctx.user_name.clone()).await;
        if user.is_none() {
            return Some(errors::form_response("UnFollow", response::Status::BackendError).await);
        }
        let user = service_response::User::get(self.user_name.clone()).await;
        if user.is_none() {
            return Some(errors::form_response("UnFollow", response::Status::BackendError).await);
        }
        let user = user.unwrap();
        let resp = user.unfollow(self.user_name.clone()).await;
        if resp.is_none() {
            return Some(errors::form_response("UnFollow", response::Status::BackendError).await);
        }
        Some(response::Response {
            operation: Some(response::response::Operation::UnFollow(
                response::UnFollow {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::ListFollowers {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("ListFollowers", response::Status::BackendError).await,
            );
        }
        let user = service_response::User::get(self.user_name.clone()).await;
        if user.is_none() {
            return Some(
                errors::form_response("ListFollowers", response::Status::BackendError).await,
            );
        }
        let user = user.unwrap();
        let followers = user.list_followers(self.page).await;
        if followers.is_none() {
            return Some(
                errors::form_response("ListFollowers", response::Status::BackendError).await,
            );
        }
        let followers = followers.unwrap();
        Some(response::Response {
            operation: Some(response::response::Operation::ListFollowers(
                response::ListFollowers {
                    status: response::Status::Success as i32,
                    message: None,
                    user_names: followers,
                },
            )),
        })
    }
}

impl request::UploadProfilePicture {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("UploadProfilePicture", response::Status::BackendError).await,
            );
        }
        let user = service_response::User::get(self.user_name.clone()).await;
        if user.is_none() {
            return Some(
                errors::form_response("UploadProfilePicture", response::Status::BackendError).await,
            );
        }
        let mut user = user.unwrap();
        if user.profile_picture.is_empty() {
            let uuid = Uuid::new_v4();
            let mut bytes = uuid.as_bytes().to_vec();
            let millis = chrono::Utc::now().timestamp_millis() as u64;
            bytes.extend_from_slice(&millis.to_be_bytes());
            user.profile_picture = bytes;
        }
        let resp = user.set_profile_picture(self.data.clone()).await;
        if resp.is_none() {
            return Some(
                errors::form_response("UploadProfilePicture", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::UpdateProfilePicture(
                response::UploadProfilePicture {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::ResetPassword {
    pub async fn handle(&self, _ctx: &mut Context) -> Option<Response> {
        // if ctx.is_acuthenticated {
        //     return Some(
        //         errors::form_response("RemoveReply", response::Status::BackendError).await,
        //     );
        // }
        Some(response::Response {
            operation: Some(response::response::Operation::ResetPassword(
                response::ResetPassword {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::RemoveProfilePicture {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("RemoveProfilePicture", response::Status::BackendError).await,
            );
        }
        let user = service_response::User::get(self.user_name.clone()).await;
        if user.is_none() {
            return Some(
                errors::form_response("RemoveProfilePicture", response::Status::BackendError).await,
            );
        }
        let user = user.unwrap();
        if user.email != ctx.email {
            return Some(
                errors::form_response("RemoveProfilePicture", response::Status::BackendError).await,
            );
        }
        let res = user.remove_profile_picture().await;
        if res.is_none() {
            return Some(
                errors::form_response("RemoveProfilePicture", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::RemoveProfilePicture(
                response::RemoveProfilePicture {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}
