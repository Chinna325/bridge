use crate::{
    Context, crypto,
    helper::{BACKEND_ERROR, POST_CANNOT_BE_EMPTY},
    operations::traits::RequestHandler,
    protos::{
        request,
        response::{self, Response, Status},
    },
};
use async_trait::async_trait;
use jbackend_runtime::TWServer;

#[async_trait]
impl RequestHandler for request::AddPost {
    fn validate(&self, _ctx: &Context) -> Result<(), String> {
        if self.post.is_none() {
            return Err(String::from(POST_CANNOT_BE_EMPTY));
        }
        Ok(())
    }

    async fn handle(&self, backend: &TWServer, ctx: &mut Context) -> Result<Response, ()> {
        let uuid = crypto::uuid();
        let post = self.post.clone().unwrap();

        if backend
            .db()
            .add_post(&uuid, &post.text, &ctx.user_name)
            .await
            .is_err()
        {
            return Ok(Self::build_response(
                Status::BackendError,
                BACKEND_ERROR.to_string(),
            ));
        }

        Ok(response::Response {
            operation: Some(response::response::Operation::AddPost(response::AddPost {
                status: Status::Success as i32,
                message: String::new(),
                post_id: uuid,
            })),
        })
    }

    fn build_response(status: Status, message: String) -> Response {
        Response {
            operation: Some(response::response::Operation::AddPost(response::AddPost {
                status: status as i32,
                message,
                post_id: Vec::new(),
            })),
        }
    }
}

#[async_trait]
impl RequestHandler for request::RemovePost {
    fn validate(&self, _ctx: &Context) -> Result<(), String> {
        if self.post_id.is_empty() {
            return Err(String::from(POST_CANNOT_BE_EMPTY));
        }
        Ok(())
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        if backend.db().remove_post(&self.post_id).await.is_err() {
            return Ok(Self::build_response(
                Status::BackendError,
                BACKEND_ERROR.to_string(),
            ));
        }

        Ok(Self::build_response(Status::Success, String::new()))
    }

    fn build_response(status: Status, message: String) -> Response {
        Response {
            operation: Some(response::response::Operation::RemovePost(
                response::RemovePost {
                    status: status as i32,
                    message,
                },
            )),
        }
    }
}

#[async_trait]
impl RequestHandler for request::GetPost {
    fn validate(&self, _ctx: &Context) -> Result<(), String> {
        if self.post_id.is_empty() {
            return Err(String::from(POST_CANNOT_BE_EMPTY));
        }
        Ok(())
    }

    async fn handle(&self, backend: &TWServer, _ctx: &mut Context) -> Result<Response, ()> {
        let (owner, text, created_at) = match backend.db().read_post(&self.post_id).await {
            Ok(post) => post,

            Err(_) => {
                return Ok(Self::build_response(
                    Status::BackendError,
                    BACKEND_ERROR.to_string(),
                ));
            }
        };

        let mut post = crate::protos::common::Post::default();
        post.created_at = created_at as u64;
        post.owner = owner;
        post.text = text;
        Ok(Response {
            operation: Some(response::response::Operation::GetPost(response::GetPost {
                status: Status::Success as i32,
                message: String::new(),
                post: Some(post),
            })),
        })
    }

    fn build_response(status: Status, message: String) -> Response {
        Response {
            operation: Some(response::response::Operation::GetPost(response::GetPost {
                status: status as i32,
                post: None,
                message,
            })),
        }
    }
}

#[async_trait]
impl RequestHandler for request::ListPosts {
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
impl RequestHandler for request::UpdatePost {
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
impl RequestHandler for request::ReactToPost {
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
impl RequestHandler for request::UndoReactToPost {
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
impl RequestHandler for request::RepostPost {
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
impl RequestHandler for request::ReplyToPost {
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
impl RequestHandler for request::EditReply {
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
impl RequestHandler for request::GetReply {
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
impl RequestHandler for request::ListReplies {
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
impl RequestHandler for request::RemoveReply {
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
