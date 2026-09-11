use crate::{
    Context,
    protos::{
        request,
        response::{Response, Status},
    },
    traits::RequestHandler,
};
use async_trait::async_trait;
use jbackend_runtime::TWServer;

#[async_trait]
impl RequestHandler for request::AddUser {
    fn validate(&self, _ctx: &Context) -> Result<(), ()> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: Option<String>) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::VerifyUser {
    fn validate(&self, _ctx: &Context) -> Result<(), ()> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: Option<String>) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::RemoveUser {
    fn validate(&self, _ctx: &Context) -> Result<(), ()> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: Option<String>) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::ChangePassword {
    fn validate(&self, _ctx: &Context) -> Result<(), ()> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: Option<String>) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::GetProfilePicture {
    fn validate(&self, _ctx: &Context) -> Result<(), ()> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: Option<String>) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::GetUser {
    fn validate(&self, _ctx: &Context) -> Result<(), ()> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: Option<String>) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::UpdateUser {
    fn validate(&self, _ctx: &Context) -> Result<(), ()> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: Option<String>) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::SignIn {
    fn validate(&self, _ctx: &Context) -> Result<(), ()> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: Option<String>) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::SignOut {
    fn validate(&self, _ctx: &Context) -> Result<(), ()> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: Option<String>) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::Follow {
    fn validate(&self, _ctx: &Context) -> Result<(), ()> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: Option<String>) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::UnFollow {
    fn validate(&self, _ctx: &Context) -> Result<(), ()> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: Option<String>) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::ListFollowers {
    fn validate(&self, _ctx: &Context) -> Result<(), ()> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: Option<String>) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::UploadProfilePicture {
    fn validate(&self, _ctx: &Context) -> Result<(), ()> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: Option<String>) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::ResetPassword {
    fn validate(&self, _ctx: &Context) -> Result<(), ()> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: Option<String>) -> Response {
        // construct your Response here
        todo!()
    }
}

#[async_trait]
impl RequestHandler for request::RemoveProfilePicture {
    fn validate(&self, _ctx: &Context) -> Result<(), ()> {
        todo!()
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        todo!()
    }

    fn build_response(status: Status, message: Option<String>) -> Response {
        // construct your Response here
        todo!()
    }
}
