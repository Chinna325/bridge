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
impl RequestHandler for request::AddPost {
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
impl RequestHandler for request::RemovePost {
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
impl RequestHandler for request::GetPost {
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
impl RequestHandler for request::ListPosts {
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
impl RequestHandler for request::UpdatePost {
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
impl RequestHandler for request::ReactToPost {
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
impl RequestHandler for request::UndoReactToPost {
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
impl RequestHandler for request::RepostPost {
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
impl RequestHandler for request::ReplyToPost {
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
impl RequestHandler for request::EditReply {
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
impl RequestHandler for request::GetReply {
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
impl RequestHandler for request::ListReplies {
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
impl RequestHandler for request::RemoveReply {
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
