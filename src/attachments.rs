use crate::{
    Context, backend,
    protos::{self, common, response, service_request, service_response},
};
struct Attachment {}
impl Attachment {
    pub async fn read(id: Vec<u8>, attachment_type: common::AttachmentType) -> Result<Vec<u8>, ()> {
        let clinet = backend::ceate_grpc_connection().await;
        let req = service_request::ServiceRequest {
            operation: Some(service_request::service_request::Operation::ReadAttchment(
                service_request::ReadAttachment {
                    uuid: id,
                    attachment_type: attachment_type as i32,
                },
            )),
        };
        let resp = req.execute(clinet).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::ReadAttchment(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
            return Ok(resp.data.clone());
        }
        Err(())
    }
    pub async fn new(req: service_request::ServiceRequest) -> Result<(), ()> {
        let client = backend::ceate_grpc_connection().await;
        let resp = req.execute(client).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::AddAttachment(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }
    pub async fn remove(attachment_id: Vec<u8>, type_of_attachment: i32) -> Result<(), ()> {
        let clinet = backend::ceate_grpc_connection().await;
        let req = service_request::ServiceRequest {
            operation: Some(
                service_request::service_request::Operation::RemoveAttachment(
                    service_request::RemoveAttachment {
                        attachment_type: type_of_attachment,
                        attachment_id,
                    },
                ),
            ),
        };
        let resp = req.execute(clinet).await.ok_or(())?;

        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::AddAttachment(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }
    pub async fn validate(id: Vec<u8>, attachment_type: common::AttachmentType) -> Result<(), ()> {
        match attachment_type {
            protos::common::AttachmentType::AttachementPost => {
                if protos::service_request::Conversation::get(id.clone())
                    .await
                    .is_err()
                {
                    return Err(());
                };
            }
            protos::common::AttachmentType::AttachmentConversation => {
                if protos::service_request::Post::from_uuid(id.clone())
                    .await
                    .is_err()
                {
                    return Err(());
                }
            }
        }
        Ok(())
    }
    pub async fn list_attachments(
        post_or_conversation: Vec<u8>,
        attachment_type: common::AttachmentType,
    ) -> Result<(Vec<String>, Vec<Vec<u8>>), ()> {
        let req = service_request::ServiceRequest {
            operation: Some(
                service_request::service_request::Operation::ListAttachments(
                    service_request::ListAttachments {
                        conversation_or_post_id: post_or_conversation,
                        attachment_type: attachment_type as i32,
                    },
                ),
            ),
        };
        let resp = req
            .execute(backend::ceate_grpc_connection().await)
            .await
            .ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::ListAttachments(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
            return Ok((resp.names.clone(), resp.attached_ids.clone()));
        }
        Err(())
    }
}

use crate::errors;
impl protos::request::AddAttachment {
    pub async fn handle(&self, ctx: &mut Context) -> Option<response::Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("AddAttachment", response::Status::BackendError).await,
            );
        }
        match Attachment::validate(self.conversation_or_post_id.clone(), self.attachment_type())
            .await
        {
            Ok(_) => {}
            Err(_) => {
                return Some(
                    errors::form_response("AddAttachment", response::Status::BackendError).await,
                );
            }
        }

        let mut uuid = uuid::Uuid::new_v4().as_bytes().to_vec();
        let millis = chrono::Utc::now().timestamp_millis() as u64;
        uuid.extend_from_slice(&millis.to_be_bytes());
        let req = service_request::AddAttachment {
            attachment_type: self.attachment_type as i32,
            data: self.data.clone(),
            conversation_or_post_id: self.conversation_or_post_id.clone(),
            shared_by: ctx.email.clone(),
            attachment_name: self.name.clone(),
            attachment_id: uuid,
        };
        let req = service_request::ServiceRequest {
            operation: Some(service_request::service_request::Operation::AddAttachment(
                req,
            )),
        };
        match Attachment::new(req).await {
            Ok(_) => {
                return Some(
                    errors::form_response("AddAttachment", response::Status::Success).await,
                );
            }
            Err(_) => {
                return Some(
                    errors::form_response("AddAttachment", response::Status::BackendError).await,
                );
            }
        }
    }
}

impl protos::request::RemoveAttachement {
    pub async fn handle(&self, ctx: &mut Context) -> Option<response::Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("RemoveAttachment", response::Status::BackendError).await,
            );
        }

        match Attachment::validate(self.conversation_or_post_id.clone(), self.attachment_type())
            .await
        {
            Ok(_) => {}
            Err(_) => {
                return Some(
                    errors::form_response("AddAttachment", response::Status::BackendError).await,
                );
            }
        }
        match Attachment::remove(self.conversation_or_post_id.clone(), self.attachment_type).await {
            Ok(_) => {
                return Some(
                    errors::form_response("RemoveAttachement", response::Status::Success).await,
                );
            }
            Err(_) => {
                return Some(
                    errors::form_response("RemoveAttachement", response::Status::BackendError)
                        .await,
                );
            }
        }
    }
}

impl protos::request::ReadAttachment {
    pub async fn handle(&self, ctx: &mut Context) -> Option<response::Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("ReadAttachment", response::Status::BackendError).await,
            );
        }
        match Attachment::read(self.uuid.clone(), self.attachment_type()).await {
            Ok(data) => {
                return Some(response::Response {
                    operation: Some(response::response::Operation::ReadAttchment(
                        response::ReadAttchment {
                            status: response::Status::Success as i32,
                            data,
                        },
                    )),
                });
            }
            Err(_) => {
                return Some(
                    errors::form_response("ReadAttachment", response::Status::BackendError).await,
                );
            }
        }
        // todo!()
    }
}
impl protos::request::ListAttachements {
    pub async fn handle(&self, ctx: &mut Context) -> Option<response::Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("ListAttachements", response::Status::BackendError).await,
            );
        }
        match Attachment::list_attachments(
            self.conversation_or_post_id.clone(),
            self.attachment_type(),
        )
        .await
        {
            Ok((names, ids)) => {
                return Some(response::Response {
                    operation: Some(response::response::Operation::ListAttachements(
                        response::ListAttachments {
                            status: response::Status::Success as i32,
                            attached_ids: ids.clone(),
                            names,
                        },
                    )),
                });
            }
            Err(_) => {
                return Some(
                    errors::form_response("ListAttachements", response::Status::BackendError).await,
                );
            }
        }
    }
}
