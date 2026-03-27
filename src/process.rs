// use std::clone;

use crate::{
    Context, backend, errors, request,
    response::{self, Response},
    service_request::{self},
    service_response,
};
use prost::Message;
use sha2::Digest;
use vortex_otp_lib::{OtpCharSet, generate_otp};
pub async fn process_request(data: Vec<u8>, ctx: &mut Context) -> Option<Vec<u8>> {
    let req = request::Request::decode(data.as_slice()).ok()?;
    let resp = req.handle(ctx).await?;
    Some(resp.encode_to_vec())
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
                return req.handle().await;
            }
            Some(request::request::Operation::AddTweet(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::ChangePassword(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::GetProfilePicture(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::GetUser(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::UpdateUser(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::RemoveProfilePicture(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::RemoveReply(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::RemoveTweet(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::GetTweet(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::SignIn(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::SignOut(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::Follow(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::UnFollow(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::ListFollowers(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::ListReplies(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::RepostTweet(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::ListTweets(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::ReplyToTweet(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::ReactToTweet(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::UndoReactToTweet(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::UpdateTweet(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::EditReply(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::GetReply(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::UpdateProfilePicture(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::ResetPassword(req)) => {
                return req.handle().await;
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
        let mut hasher = sha2::Sha256::new();
        hasher.update(password);
        let password_hash = hasher.finalize().to_vec();
        let mut hasher = sha2::Sha256::new();
        hasher.update(self.email.clone());
        let key = hasher.finalize().to_vec();
        let redis_object = service_request::RedisObject {
            email: email.clone(),
            user_name: user_name.clone(),
            email_otp: Some(email_otp),
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
            operation: Some(response::response::Operation::AddUser(response::AddUser {})),
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
        if object.email_otp.unwrap_or_default() != self.email_otp.clone() {
            return Some(errors::form_response("VerifyUser", response::Status::BackendError).await);
        }
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
        Some(response::Response {
            operation: Some(response::response::Operation::VerifyUser(
                response::VerifyUser {},
            )),
        })
        // todo!()
    }
}

impl request::RemoveUser {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::AddTweet {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::ChangePassword {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::GetProfilePicture {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::GetUser {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::UpdateUser {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::RemoveReply {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::RemoveTweet {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::GetTweet {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::SignIn {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::SignOut {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::Follow {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::UnFollow {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::ListFollowers {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::ListReplies {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::RepostTweet {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::ListTweets {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::ReplyToTweet {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::ReactToTweet {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::UndoReactToTweet {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::UpdateTweet {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::EditReply {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::GetReply {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::UploadProfilePicture {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::ResetPassword {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::RemoveProfilePicture {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}
