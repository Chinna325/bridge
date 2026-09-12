use crate::{
    Context,
    crypto::{self, sha_256},
    operations::traits::RequestHandler,
    protos::{
        self, common, request,
        response::{self, Response, Status},
    },
};
use async_trait::async_trait;

use jbackend_runtime::{TWServer, errors::StatusCode};
use prost::Message;

#[async_trait]
impl RequestHandler for request::AddUser {
    fn validate(&self, ctx: &Context) -> Result<(), String> {
        if ctx.is_acuthenticated {
            return Err(String::from(""));
        }
        if self.user_email.is_empty() {
            return Err(String::from(""));
        }

        if self.user_name.is_empty() {
            return Err(String::from(""));
        }

        if self.password.is_empty() {
            return Err(String::from(""));
        }
        Ok(())
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        let error = match backend.db().user_name_exists(&self.user_name).await {
            Ok(e) => e,
            Err(e) => e,
        };

        match error.status {
            StatusCode::StatusErrorUserNameAlreadyExists
            | StatusCode::StatusErrorDBQueryFailure => {
                return Ok(Self::build_response(Status::BackendError, error.message));
            }
            _ => {}
        }

        let otp = match crypto::generate_top() {
            Ok(otp) => otp,
            Err(e) => {
                return Ok(Self::build_response(Status::BackendError, e));
            }
        };

        let redis_object = protos::common::RedisObject {
            user_name: self.user_name.clone(),
            email: self.user_email.clone(),
            opt: otp,
            password: self.password.clone(),
        };

        let key = crypto::sha_256(&self.user_email);

        let _ = backend
            .cache()
            .write(&hex::encode(key), redis_object.encode_to_vec())
            .await;

        Ok(Self::build_response(Status::Success, String::new()))
    }

    fn build_response(status: Status, message: String) -> Response {
        Response {
            operation: Some(response::response::Operation::AddUser(response::AddUser {
                status: status as i32,
                message,
                otp: String::new(),
            })),
        }
    }
}

#[async_trait]
impl RequestHandler for request::VerifyUser {
    fn validate(&self, ctx: &Context) -> Result<(), String> {
        if ctx.is_acuthenticated {
            return Err(String::new());
        }
        if self.user_email.is_empty() {
            return Err(String::new());
        }

        if self.email_otp.is_empty() {
            return Err(String::new());
        }

        Ok(())
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        let key = sha_256(&self.user_email);
        let data = match backend.cache().read(&hex::encode(key)).await {
            Ok(data) => data,
            Err(e) => {
                return Ok(Self::build_response(Status::BackendError, e.message));
            }
        };

        let redis_object = match common::RedisObject::decode(data.as_slice()) {
            Ok(obect) => obect,
            Err(_) => {
                return Ok(Self::build_response(Status::BackendError, String::new()));
            }
        };

        if redis_object.email != self.user_email || redis_object.opt != self.email_otp {
            return Ok(Self::build_response(Status::BackendError, String::new()));
        }

        todo!()
    }

    fn build_response(status: Status, message: String) -> Response {
        Response {
            operation: Some(response::response::Operation::VerifyUser(
                response::VerifyUser {
                    status: status as i32,
                    message,
                },
            )),
        }
    }
}

#[async_trait]
impl RequestHandler for request::RemoveUser {
    fn validate(&self, _ctx: &Context) -> Result<(), String> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: String) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::ChangePassword {
    fn validate(&self, _ctx: &Context) -> Result<(), String> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: String) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::GetProfilePicture {
    fn validate(&self, _ctx: &Context) -> Result<(), String> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: String) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::GetUser {
    fn validate(&self, _ctx: &Context) -> Result<(), String> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: String) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::UpdateUser {
    fn validate(&self, _ctx: &Context) -> Result<(), String> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: String) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::SignIn {
    fn validate(&self, _ctx: &Context) -> Result<(), String> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: String) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::SignOut {
    fn validate(&self, _ctx: &Context) -> Result<(), String> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: String) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::Follow {
    fn validate(&self, _ctx: &Context) -> Result<(), String> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: String) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::UnFollow {
    fn validate(&self, _ctx: &Context) -> Result<(), String> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: String) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::ListFollowers {
    fn validate(&self, _ctx: &Context) -> Result<(), String> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: String) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::UploadProfilePicture {
    fn validate(&self, _ctx: &Context) -> Result<(), String> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: String) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::ResetPassword {
    fn validate(&self, _ctx: &Context) -> Result<(), String> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: String) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::RemoveProfilePicture {
    fn validate(&self, _ctx: &Context) -> Result<(), String> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: String) -> Response {
        // construct your Response here
        todo!()
    }
}
