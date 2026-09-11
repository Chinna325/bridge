use async_trait::async_trait;

use crate::Context;
use crate::protos::response::{Response, Status};
use jbackend_runtime::TWServer;

#[async_trait]
pub trait RequestHandler {
    fn validate(&self, ctx: &Context) -> Result<(), ()>;

    async fn handle(&self, backend: &TWServer, ctx: &mut Context) -> Result<Response, ()>;

    fn build_response(status: Status, message: Option<String>) -> Response;
}
