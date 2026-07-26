// use crate::request::Post;
use crate::errors;
use crate::protos::common;
use crate::protos::response::Response;
use crate::{Context, protos::response};
use crate::{
    backend,
    protos::request,
    protos::service_request::{self, ServiceRequest},
    protos::service_response::{self},
};
use chrono::Utc;
use prost::Message;
use uuid::Uuid;

impl service_request::Post {
    pub async fn new(
        &self,
        type_of_add: service_request::PostAdd,
        post_data: Vec<u8>,
    ) -> Result<(), ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::AddPost(
                service_request::AddPost {
                    user_email: self.owner.clone(),
                    post_data: post_data,
                    post_add: type_of_add as i32,
                    post_id: self.post_id.clone(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::AddPost(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }
    pub async fn from_uuid(uuid: Vec<u8>) -> Result<Self, ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::GetPost(
                service_request::GetPost { post_id: uuid },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::GetPost(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
            let post_data = resp.post_data.clone();
            let post = service_request::Post::decode(post_data.as_slice());
            if post.is_err() {
                return Err(());
            }
            return Ok(post.unwrap());
        }
        Err(())
    }
    pub async fn remove(&self, post_remove: service_request::PostRemove) -> Result<(), ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::RemovePost(
                service_request::RemovePost {
                    post_remove: post_remove as i32,
                    post_id: self.post_id.clone(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::RemovePost(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }
    pub async fn update(&mut self) -> Result<(), ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::UpdatePost(
                service_request::UpdatePost {
                    post_data: self.clone().encode_to_vec(),
                    post_id: self.post_id.clone(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::UpdatePost(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }
    pub async fn list(user_email: String) -> Result<Vec<Vec<u8>>, ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::ListPosts(
                service_request::ListPosts { user_email },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::ListPosts(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
            return Ok(resp.post_ids);
        }
        Err(())
    }

    pub async fn react_to_post(&mut self) -> Result<(), ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::PostReact(
                service_request::PostReact {
                    post_id: self.post_id.clone(),
                    post_data: self.clone().encode_to_vec(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::PostReact(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }

    pub async fn list_replies(
        &self,
        parent_id: Vec<u8>,
    ) -> Result<Vec<service_response::Reply>, ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::ListReplies(
                service_request::ListReplies {
                    post_id: self.post_id.clone(),
                    parent_id: parent_id.clone(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::ListReplies(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
            let replies = resp.replies.clone();
            return Ok(replies);
        }
        Err(())
    }
    pub fn form(post: common::Post) -> Self {
        Self {
            post_id: post.post_id.clone(),
            text: post.text.clone(),
            created_at: Utc::now().timestamp() as u64,
            owner: post.owner.clone(),
            public_metrics: Some(service_request::PublicMetrics::default()),
            hashtags: post.hashtags.clone(),
            emails: post.user_emails.clone(),
        }
    }
}

impl service_request::Reply {
    pub async fn new(&self) -> Result<(), ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::AddReply(
                service_request::AddReply {
                    reply: Some(self.clone()),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::AddReply(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }

    pub async fn remove(&self) -> Result<(), ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::RemoveReply(
                service_request::RemoveReply {
                    reply_id: self.reply_id.clone(),
                    parent_id: self.parent_id.clone(),
                    post_id: self.post_id.clone(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::RemoveReply(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }

    pub async fn update(&mut self) -> Result<(), ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::UpdateReply(
                service_request::UpdateReply {
                    reply_id: self.reply_id.clone(),
                    parent_id: self.parent_id.clone(),
                    text: self.text.clone(),
                    hash_tags: self.hash_tags.clone(),
                    user_email: self.user_emails.clone(),
                    post_id: self.post_id.clone(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::UpdateReply(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }

    pub async fn from_uuid(
        post_id: Vec<u8>,
        reply_id: Vec<u8>,
        parent_id: Vec<u8>,
    ) -> Result<Self, ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::GetReply(
                service_request::GetReply {
                    reply_id: reply_id.clone(),
                    parent_id: parent_id.clone(),
                    post_id: post_id.clone(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::GetReply(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
            let reply = resp.reply.clone();
            if reply.is_none() {
                return Err(());
            }
            let reply = reply.unwrap();
            return Ok(Self {
                post_id,
                email: reply.user_email.clone(),
                text: reply.text.clone(),
                reply_id,
                parent_id,
                likes: reply.likes,
                dislikes: reply.dislikes,
                created_at: reply.created_at,
                hash_tags: reply.hash_tags.clone(),
                user_emails: reply.user_emails.clone(),
            });
        }
        Err(())
    }
    pub async fn react_reply(&self) -> Result<(), ()> {
        Ok(())
    }

    pub fn from(reply: request::Reply) -> Self {
        Self {
            post_id: reply.post_id.clone(),
            email: reply.user_email.clone(),
            text: reply.text.clone(),
            reply_id: reply.reply_id.clone(),
            parent_id: reply.parent_id.clone(),
            likes: reply.likes,
            dislikes: reply.dislikes,
            created_at: 0_u64,
            hash_tags: Vec::new(),
            user_emails: Vec::new(),
        }
    }
}

impl request::AddPost {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("AddPost", response::Status::BackendError).await);
        }
        let post = self.post.clone();
        if post.is_none() {
            return Some(errors::form_response("AddPost", response::Status::BackendError).await);
        }
        let post = post.unwrap();
        let mut post_id = post.post_id.clone();
        let mut post_add = service_request::PostAdd::Add;
        if post_id.is_empty() {
            let uuid = Uuid::new_v4();
            let mut bytes = uuid.as_bytes().to_vec();
            let millis = chrono::Utc::now().timestamp_millis() as u64;
            bytes.extend_from_slice(&millis.to_be_bytes());
            post_id = bytes;
            let mut post = service_request::Post::form(post.clone());
            post.post_id = post_id.clone();
            post.owner = ctx.email.clone();
            let resp = post.new(post_add, post.encode_to_vec()).await;
            if resp.is_err() {
                return Some(
                    errors::form_response("AddPost", response::Status::BackendError).await,
                );
            }
        } else {
            let post = service_request::Post::from_uuid(post_id.clone()).await;
            if post.is_err() {
                return Some(
                    errors::form_response("AddPost", response::Status::BackendError).await,
                );
            }
            let post = post.unwrap();
            post_add = service_request::PostAdd::Repost;
            let resp = post.new(post_add, Vec::new()).await;
            if resp.is_err() {
                return Some(
                    errors::form_response("AddPost", response::Status::BackendError).await,
                );
            }
        }
        Some(response::Response {
            operation: Some(response::response::Operation::AddPost(response::AddPost {
                status: response::Status::Success as i32,
                message: None,
                post_id: post_id.clone(),
            })),
        })
    }
}

impl request::RemovePost {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("RemovePost", response::Status::BackendError).await);
        }
        let post = service_request::Post::from_uuid(self.post_id.clone()).await;
        if post.is_err() {
            return Some(errors::form_response("RemovePost", response::Status::BackendError).await);
        }
        let post = post.unwrap();
        let mut post_remove = service_request::PostRemove::OwnPost;
        if ctx.email != post.owner {
            post_remove = service_request::PostRemove::RepostedPost;
        }
        let resp = post.remove(post_remove).await;
        if resp.is_err() {
            return Some(errors::form_response("RemovePost", response::Status::BackendError).await);
        }
        Some(response::Response {
            operation: Some(response::response::Operation::RemovePost(
                response::RemovePost {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::GetPost {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("GetPost", response::Status::BackendError).await);
        }
        let post = service_request::Post::from_uuid(self.post_id.clone()).await;
        if post.is_err() {
            return Some(errors::form_response("GetPost", response::Status::BackendError).await);
        }
        let post = post.unwrap();
        let metrics = post.public_metrics.clone().unwrap();
        let public_metrics = common::PublicMetrics {
            re_post_count: metrics.repost_count,
            reply_count: metrics.reply_count,
            like_count: metrics.like_count,
            quote_count: metrics.quote_count,
            bookmark_count: metrics.bookmark_count,
            impression_count: metrics.impression_count,
        };
        let post = common::Post {
            post_id: self.post_id.clone(),
            text: post.text.clone(),
            created_at: post.created_at,
            owner: post.owner.clone(),
            hashtags: post.hashtags.clone(),
            user_emails: post.emails.clone(),
            public_metrics: Some(public_metrics),
        };
        Some(response::Response {
            operation: Some(response::response::Operation::GetPost(response::GetPost {
                status: response::Status::Success as i32,
                message: None,
                post: Some(post),
            })),
        })
    }
}

impl request::ListPosts {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("ListPosts", response::Status::BackendError).await);
        }
        let posts = service_request::Post::list(self.user_email.clone()).await;
        if posts.is_err() {
            return Some(errors::form_response("ListPosts", response::Status::BackendError).await);
        }
        let posts = posts.unwrap();
        Some(response::Response {
            operation: Some(response::response::Operation::ListPosts(
                response::ListPosts {
                    status: response::Status::Success as i32,
                    message: None,
                    posts: posts,
                },
            )),
        })
    }
}

impl request::UpdatePost {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("UpdatePost", response::Status::BackendError).await);
        }
        let post = service_request::Post::from_uuid(self.post_id.clone()).await;
        if post.is_err() {
            return Some(errors::form_response("UpdatePost", response::Status::BackendError).await);
        }
        let mut post = post.unwrap();
        if post.owner != ctx.email {
            return Some(errors::form_response("UpdatePost", response::Status::BackendError).await);
        }
        post.hashtags = self.hash_tags.clone();
        post.emails = self.user_emails.clone();
        post.text = self.text.clone();
        let resp = post.update().await;
        if resp.is_err() {
            return Some(errors::form_response("UpdatePost", response::Status::BackendError).await);
        }
        Some(response::Response {
            operation: Some(response::response::Operation::UpdatePost(
                response::UpdatePost {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl common::PostReact {
    pub fn from(number: i32) -> Self {
        match number {
            0 => Self::Like,
            1 => Self::Love,
            2 => Self::DisLike,
            _ => Self::Like,
        }
    }
}

impl common::UndoPostReact {
    pub fn from(number: i32) -> Self {
        match number {
            0 => Self::UndoLike,
            1 => Self::UndoDisLike,
            2 => Self::UndoLove,
            _ => Self::UndoLike,
        }
    }
}
impl request::ReactToPost {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("ReactToPost", response::Status::BackendError).await,
            );
        }
        let post = service_request::Post::from_uuid(self.post_id.clone()).await;
        if post.is_err() {
            return Some(
                errors::form_response("ReactToPost", response::Status::BackendError).await,
            );
        }
        let mut post = post.unwrap();
        let metrics = post.public_metrics.clone();
        let mut metrics = metrics.unwrap();
        let operation = common::PostReact::from(self.post_react);
        let resp = match operation {
            common::PostReact::Like => {
                metrics.like_count += 1;
                post.public_metrics = Some(metrics.clone());
                post.react_to_post().await
            }
            common::PostReact::Love => {
                metrics.love_count += 1;
                post.public_metrics = Some(metrics.clone());
                post.react_to_post().await
            }
            common::PostReact::DisLike => {
                metrics.dislike_count += 1;
                post.public_metrics = Some(metrics.clone());
                post.react_to_post().await
            }
        };
        if resp.is_err() {
            return Some(
                errors::form_response("ReactToPost", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::ReactToPost(
                response::ReactToPost {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::UndoReactToPost {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("UndoReactToPost", response::Status::BackendError).await,
            );
        }
        let post = service_request::Post::from_uuid(self.post_id.clone()).await;
        if post.is_err() {
            return Some(
                errors::form_response("UndoReactToPost", response::Status::BackendError).await,
            );
        }
        let mut post = post.unwrap();
        let metrics = post.public_metrics.clone();
        let mut metrics = metrics.unwrap();
        let operation = common::UndoPostReact::from(self.post_react);
        let resp = match operation {
            common::UndoPostReact::UndoDisLike => {
                metrics.dislike_count -= 1;
                post.public_metrics = Some(metrics.clone());
                post.react_to_post().await
            }
            common::UndoPostReact::UndoLike => {
                metrics.like_count -= 1;
                post.public_metrics = Some(metrics.clone());
                post.react_to_post().await
            }
            common::UndoPostReact::UndoLove => {
                metrics.love_count -= 1;
                post.public_metrics = Some(metrics.clone());
                post.react_to_post().await
            }
        };
        if resp.is_err() {
            return Some(
                errors::form_response("UndoReactToPost", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::UndoReactToPost(
                response::UndoReactToPost {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::RepostPost {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("RepostPost", response::Status::BackendError).await);
        }
        let post = service_request::Post::from_uuid(self.post_id.clone()).await;
        if post.is_err() {
            return Some(errors::form_response("RepostPost", response::Status::BackendError).await);
        }
        let mut post = post.unwrap();
        if let Some(metrics) = post.public_metrics.as_mut() {
            metrics.repost_count += 1;
        }
        let resp = post
            .new(service_request::PostAdd::Repost, post.encode_to_vec())
            .await;
        if resp.is_err() {
            return Some(errors::form_response("RepostPost", response::Status::BackendError).await);
        }
        Some(response::Response {
            operation: Some(response::response::Operation::RepostPost(
                response::RepostPost {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::ReplyToPost {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("ReplyToPost", response::Status::BackendError).await,
            );
        }
        if self.reply.clone().is_none() {
            return Some(
                errors::form_response("ReplyToPost", response::Status::BackendError).await,
            );
        }
        let reply = self.reply.clone().unwrap();
        let mut reply = service_request::Reply::from(reply);
        reply.created_at = chrono::Utc::now().timestamp() as u64;
        let uuid = Uuid::new_v4();
        let mut bytes = uuid.as_bytes().to_vec();
        let millis = chrono::Utc::now().timestamp_millis() as u64;
        bytes.extend_from_slice(&millis.to_be_bytes());
        reply.reply_id = bytes.clone();
        let resp = reply.new().await;
        if resp.is_err() {
            return Some(
                errors::form_response("ReplyToPost", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::ReplyToPost(
                response::ReplyToPost {
                    status: response::Status::Success as i32,
                    message: None,
                    reply_id: bytes,
                },
            )),
        })
    }
}

impl request::EditReply {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("EditReply", response::Status::BackendError).await);
        }
        let post = service_request::Post::from_uuid(self.post_id.clone()).await;
        if post.is_err() {
            return Some(errors::form_response("EditReply", response::Status::BackendError).await);
        }
        let post = post.unwrap();
        let reply = service_request::Reply::from_uuid(
            post.post_id.clone(),
            self.reply_id.clone(),
            self.parent_id.clone(),
        )
        .await;
        if reply.is_err() {
            return Some(errors::form_response("EditReply", response::Status::BackendError).await);
        }
        let mut reply = reply.unwrap();
        if reply.email != ctx.email {
            return Some(errors::form_response("EditReply", response::Status::BackendError).await);
        }
        reply.text = self.text.clone();
        reply.hash_tags = self.hash_tags.clone();
        reply.user_emails = self.user_emails.clone();
        let resp = reply.update().await;
        if resp.is_err() {
            return Some(errors::form_response("EditReply", response::Status::BackendError).await);
        }
        Some(response::Response {
            operation: Some(response::response::Operation::EditReply(
                response::EditReply {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl common::Reply {
    pub fn from(reply: service_response::Reply) -> Self {
        Self {
            post_id: reply.post_id.clone(),
            user_email: reply.user_email.clone(),
            text: reply.text.clone(),
            reply_id: reply.reply_id.clone(),
            parent_id: reply.parent_id.clone(),
            likes: reply.likes,
            dislikes: reply.dislikes,
        }
    }
    pub fn from_request(reply: service_request::Reply) -> Self {
        Self {
            post_id: reply.post_id.clone(),
            user_email: reply.email.clone(),
            text: reply.text.clone(),
            reply_id: reply.reply_id.clone(),
            parent_id: reply.parent_id.clone(),
            likes: reply.likes,
            dislikes: reply.dislikes,
        }
    }
}
impl request::GetReply {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("GetReply", response::Status::BackendError).await);
        }
        let post = service_request::Post::from_uuid(self.post_id.clone()).await;
        if post.is_err() {
            return Some(errors::form_response("GetReply", response::Status::BackendError).await);
        }
        let post = post.unwrap();
        let reply = service_request::Reply::from_uuid(
            post.post_id.clone(),
            self.reply_id.clone(),
            self.parent_id.clone(),
        )
        .await;
        if reply.is_err() {
            return Some(errors::form_response("GetReply", response::Status::BackendError).await);
        }
        let reply = reply.unwrap();
        let reply = common::Reply::from_request(reply);
        Some(response::Response {
            operation: Some(response::response::Operation::GetReply(
                response::GetReply {
                    status: response::Status::Success as i32,
                    message: None,
                    reply: Some(reply),
                },
            )),
        })
    }
}

impl request::ListReplies {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("ListReplies", response::Status::BackendError).await,
            );
        }
        let post = service_request::Post::from_uuid(self.post_id.clone()).await;
        if post.is_err() {
            return Some(
                errors::form_response("ListReplies", response::Status::BackendError).await,
            );
        }
        let post = post.unwrap();
        let replies = post.list_replies(self.parent_id.clone()).await;
        if replies.is_err() {
            return Some(
                errors::form_response("ListReplies", response::Status::BackendError).await,
            );
        }
        let replies = replies.unwrap();
        let mut objects = Vec::new();
        for reply in replies {
            objects.push(common::Reply::from(reply));
        }
        Some(response::Response {
            operation: Some(response::response::Operation::ListReplies(
                response::ListReplies {
                    status: response::Status::Success as i32,
                    message: None,
                    replies: Vec::new(),
                },
            )),
        })
    }
}

impl request::RemoveReply {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("RemoveReply", response::Status::BackendError).await,
            );
        }
        let post = service_request::Post::from_uuid(self.post_id.clone()).await;
        if post.is_err() {
            return Some(
                errors::form_response("RemoveReply", response::Status::BackendError).await,
            );
        }
        let post = post.unwrap();
        let reply = service_request::Reply::from_uuid(
            post.post_id.clone(),
            self.reply_id.clone(),
            self.parent_id.clone(),
        )
        .await;
        if reply.is_err() {
            return Some(
                errors::form_response("RemoveReply", response::Status::BackendError).await,
            );
        }
        let reply = reply.unwrap();
        if reply.email != ctx.email.clone() {
            return Some(
                errors::form_response("RemoveReply", response::Status::BackendError).await,
            );
        }
        let resp = reply.remove().await;
        if resp.is_err() {
            return Some(
                errors::form_response("RemoveReply", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::RemoveReply(
                response::RemoveReply {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}
