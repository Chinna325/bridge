use crate::{
    Context,
    protos::{self, response},
};

use crate::errors;
impl protos::request::AddAttachment {
    pub async fn handle(&self, ctx: &mut Context) -> Option<response::Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("AddAttachment", response::Status::BackendError).await,
            );
        }
        let found = match self.attachment_type() {
            protos::common::AttachmentType::AttachementPost => {
                protos::service_request::Conversation::get(self.conversation_or_post_id.clone())
                    .await
                    .is_err()
            }
            protos::common::AttachmentType::AttachmentConversation => false,
        };
        todo!()
    }
}

impl protos::request::RemoveAttachement {
    pub async fn handle(&self, ctx: &mut Context) -> Option<response::Response> {
        todo!()
    }
}

impl protos::request::ReadAttachment {
    pub async fn handle(&self, ctx: &mut Context) -> Option<response::Response> {
        todo!()
    }
}
impl protos::request::ListAttachements {
    pub async fn handle(&self, ctx: &mut Context) -> Option<response::Response> {
        todo!()
    }
}
