use crate::{
    Context,
    crypto::{self, sha_256},
    helper::{
        ALREADY_AUTHENTICATED, BACKEND_ERROR, INVALID_EMAIL, INVALID_OTP, PASSWORD_CANNOT_BE_EMPTY,
        USER_NAME_CANNOT_BE_EMPTY,
    },
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
            return Err(String::from(ALREADY_AUTHENTICATED));
        }
        if self.user_email.is_empty() {
            return Err(String::from(INVALID_EMAIL));
        }

        if self.user_name.is_empty() {
            return Err(String::from(USER_NAME_CANNOT_BE_EMPTY));
        }

        if self.password.is_empty() {
            return Err(String::from(PASSWORD_CANNOT_BE_EMPTY));
        }
        Ok(())
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        let error = match backend.db().user_name_exists(&self.user_name).await {
            Ok(e) => e,
            Err(e) => e,
        };

        match error.status {
            StatusCode::StatusErrorUserNameAlreadyExists => {
                return Ok(Self::build_response(
                    Status::UserNameAlreadyExists,
                    error.message,
                ));
            }

            StatusCode::StatusErrorDBQueryFailure => {
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
            return Err(String::from(ALREADY_AUTHENTICATED));
        }
        if self.user_email.is_empty() {
            return Err(String::from(INVALID_EMAIL));
        }

        if self.email_otp.is_empty() {
            return Err(String::from(INVALID_OTP));
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
                return Ok(Self::build_response(
                    Status::BackendError,
                    String::from(BACKEND_ERROR),
                ));
            }
        };

        if redis_object.email != self.user_email {
            return Ok(Self::build_response(
                Status::BackendError,
                String::from(INVALID_EMAIL),
            ));
        }

        if redis_object.opt != self.email_otp {
            return Ok(Self::build_response(
                Status::InvalidOtp,
                String::from(INVALID_OTP),
            ));
        }

        let password = sha_256(&redis_object.password);
        match backend
            .db()
            .add_user(&redis_object.user_name, &self.user_email, &password)
            .await
        {
            Ok(_) => {}
            Err(_) => {
                return Ok(Self::build_response(
                    Status::BackendError,
                    String::from(BACKEND_ERROR),
                ));
            }
        }

        Ok(Self::build_response(Status::Success, String::new()))
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
        if self.user_name.is_empty() {
            return Err(String::from(USER_NAME_CANNOT_BE_EMPTY));
        }
        Ok(())
    }

    async fn handle(&self, backend: &TWServer, ctx: &mut Context) -> Result<Response, ()> {
        match backend.db().follow(&self.user_name, &ctx.user_name).await {
            Ok(_) => {}
            Err(e) => {
                return Ok(Self::build_response(Status::BackendError, e.message));
            }
        }

        match backend.db().follow(&self.user_name, &ctx.user_name).await {
            Ok(_) => {}
            Err(e) => {
                return Ok(Self::build_response(Status::BackendError, e.message));
            }
        }
        Ok(Self::build_response(Status::Success, String::new()))
    }

    fn build_response(status: Status, message: String) -> Response {
        Response {
            operation: Some(response::response::Operation::Follow(response::Follow {
                status: status as i32,
                message,
            })),
        }
    }
}

#[async_trait]
impl RequestHandler for request::UnFollow {
    fn validate(&self, _ctx: &Context) -> Result<(), String> {
        if self.user_name.is_empty() {
            return Err(String::from(USER_NAME_CANNOT_BE_EMPTY));
        }
        Ok(())
    }

    async fn handle(&self, backend: &TWServer, ctx: &mut Context) -> Result<Response, ()> {
        match backend.db().read_user(&self.user_name).await {
            Ok(_) => {}
            Err(e) => {
                return Ok(Self::build_response(Status::BackendError, e.message));
            }
        }

        match backend
            .db()
            .un_follow(&self.user_name, &ctx.user_name)
            .await
        {
            Ok(_) => {}
            Err(e) => {
                return Ok(Self::build_response(Status::BackendError, e.message));
            }
        }

        Ok(Self::build_response(Status::Success, String::new()))
    }

    fn build_response(status: Status, message: String) -> Response {
        Response {
            operation: Some(response::response::Operation::UnFollow(
                response::UnFollow {
                    status: status as i32,
                    message,
                },
            )),
        }
    }
}

#[async_trait]
impl RequestHandler for request::ListFollowers {
    fn validate(&self, _ctx: &Context) -> Result<(), String> {
        Ok(())
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        let result = match self.ltype() {
            protos::common::LType::Followers => {
                backend
                    .db()
                    .list_followers(&self.user_name, self.last_serial, 100)
                    .await
            }
            protos::common::LType::Followings => {
                backend
                    .db()
                    .list_followings(&self.user_name, self.last_serial, 100)
                    .await
            }
            _ => {
                return Ok(Self::build_response(
                    Status::BackendError,
                    BACKEND_ERROR.to_string(),
                ));
            }
        };

        let (users, last_serial) = match result {
            Ok(info) => info,
            Err(_) => {
                return Ok(Self::build_response(
                    Status::BackendError,
                    BACKEND_ERROR.to_string(),
                ));
            }
        };

        Ok(response::Response {
            operation: Some(response::response::Operation::ListFollowers(
                response::ListFollowers {
                    status: Status::Success as i32,
                    message: String::new(),
                    user_names: users,
                    last_serial,
                },
            )),
        })
    }

    fn build_response(status: Status, message: String) -> Response {
        Response {
            operation: Some(response::response::Operation::ListFollowers(
                response::ListFollowers {
                    status: status as i32,
                    message,
                    user_names: Vec::new(),
                    last_serial: 0,
                },
            )),
        }
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
