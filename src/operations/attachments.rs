use crate::operations::traits::RequestHandler;
use crate::{
    Context,
    protos::{
        self,
        response::{Response, Status},
    },
};
use async_trait::async_trait;
use jbackend_runtime::TWServer;

#[async_trait]
impl RequestHandler for protos::request::AddAttachment {
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
impl RequestHandler for protos::request::RemoveAttachement {
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
impl RequestHandler for protos::request::ReadAttachment {
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
impl RequestHandler for protos::request::ListAttachements {
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
