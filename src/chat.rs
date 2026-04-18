use crate::{
    Context, backend, request,
    response::{self, Response},
    service_request::ServiceRequest,
};
use crate::{errors, service_request, service_response};
use chrono::Utc;
use uuid::Uuid;
pub const MAX_NAME: u64 = 32;
impl service_request::Message {
    pub async fn new(&self) -> Result<(), ()> {
        let req = service_request::ServiceRequest {
            operation: Some(service_request::service_request::Operation::SendMessage(
                service_request::SendMessage {
                    message: Some(self.clone()),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::SendMessage(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }

    pub async fn get(chat_id: Vec<u8>, message_id: u64) -> Result<Self, ()> {
        let req = service_request::ServiceRequest {
            operation: Some(service_request::service_request::Operation::GetMessage(
                service_request::GetMessage {
                    chat_id: chat_id.clone(),
                    message_id: message_id,
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::GetMessage(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
            let message = resp.message;
            if message.is_none() {
                return Err(());
            }
            let message = message.unwrap();
            return Ok(Self {
                owner: message.owner.clone(),
                content: message.content.clone(),
                created_at: message.created_at,
                message_id: message_id,
                chat_id: chat_id.clone(),
            });
        }
        Err(())
    }
    pub async fn update(&mut self) -> Result<(), ()> {
        let req = service_request::ServiceRequest {
            operation: Some(service_request::service_request::Operation::EditMessage(
                service_request::EditMessage {
                    chat_id: self.chat_id.clone(),
                    content: self.content.clone(),
                    message_id: self.message_id.clone(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::EditMessage(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }

    pub async fn remove(&self, remove: service_request::MessageRemove) -> Result<(), ()> {
        let req = service_request::ServiceRequest {
            operation: Some(service_request::service_request::Operation::RemoveMessage(
                service_request::RemoveMessage {
                    chat_id: self.chat_id.clone(),
                    user_name: self.owner.clone(),
                    message_id: self.message_id.clone(),
                    message_remove: remove as i32,
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::RemoveMessage(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }

    pub async fn read(&self, user_name: String) -> Result<(), ()> {
        let req = service_request::ServiceRequest {
            operation: Some(service_request::service_request::Operation::ReadMessage(
                service_request::ReadMessage {
                    chat_id: self.chat_id.clone(),
                    message_id: self.message_id.clone(),
                    user_name: user_name.clone(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::ReadMessage(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }
}

impl service_request::Chat {
    pub async fn new(&self) -> Result<(), ()> {
        let req = service_request::ServiceRequest {
            operation: Some(
                service_request::service_request::Operation::CreateOneToOneChat(
                    service_request::CreateOneToOneChat {
                        chat: Some(self.clone()),
                    },
                ),
            ),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::ReadMessage(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }

    pub async fn get(chat_id: Vec<u8>) -> Result<Self, ()> {
        let req = service_request::ServiceRequest {
            operation: Some(service_request::service_request::Operation::GetChat(
                service_request::GetChat {
                    chat_id: chat_id.clone(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::GetChat(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
            let chat = resp.chat.clone();
            if chat.is_none() {
                return Err(());
            }
            let chat = chat.unwrap();
            return Ok(Self {
                chat_id: chat_id.clone(),
                user_name: chat.user_name.clone(),
                last_message_id: chat.last_message_id,
            });
        }
        Err(())
    }

    pub async fn update(&mut self) -> Result<(), ()> {
        todo!()
    }

    pub async fn remove(&self, message_id: u64) -> Result<(), ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::ClearChat(
                service_request::ClearChat {
                    chat_id: self.chat_id.clone(),
                    user_name: self.user_name.clone(),
                    message_id: message_id,
                },
            )),
        };

        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::ClearChat(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }

    pub async fn list_messages(&self) -> Result<(), ()> {
        todo!()
    }
}

impl service_request::Group {
    pub async fn new(&self) -> Result<(), ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::CreateGroup(
                service_request::CreateGroup {
                    group: Some(self.clone()),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::CreateGroup(resp)),
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
            operation: Some(service_request::service_request::Operation::RemoveGroup(
                service_request::RemoveGroup {
                    group_id: self.group_id.clone(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::RemoveGroup(resp)),
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
            operation: Some(service_request::service_request::Operation::UpdateGroup(
                service_request::UpdateGroup {
                    // groupd_id: self.group_id.clone(),
                    // user_names: self.users.clone(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::UpdateGroup(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }
    pub async fn get(group_id: Vec<u8>) -> Result<Self, ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::GetGroup(
                service_request::GetGroup {
                    group_id: group_id.clone(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::GetGroup(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
            let group = resp.group.clone();
            if group.is_none() {
                return Err(());
            }
            let group = group.unwrap();
            return Ok(Self {
                group_id: group_id.clone(),
                users: group.users.clone(),
                created_at: group.created_at,
                created_by: group.created_by.clone(),
                group_name: group.group_name.clone(),
                last_message_at: group.last_message_at,
                last_message_id: group.last_message_id,
            });
        }
        Err(())
    }
    pub async fn list(user_name: String) -> Result<Vec<service_response::Group>, ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::ListGroups(
                service_request::ListGroups {
                    user_name: user_name,
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::ListGroups(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
            return Ok(resp.groups.clone());
        }
        Err(())
    }

    pub async fn add_or_remove_user(&mut self) -> Result<(), ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::AddUserToGroup(
                service_request::AddUserToGroup {
                    groupd_id: self.group_id.clone(),
                    user_names: self.users.clone(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::AddUserToGroup(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }
}

impl request::CreateOneToOneChat {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("CreateOneToOneChat", response::Status::BackendError).await,
            );
        }
        let user = service_response::User::get(self.user_name.clone()).await;
        if user.is_none() {
            return Some(
                errors::form_response("CreateOneToOneChat", response::Status::BackendError).await,
            );
        }
        let uuid = Uuid::new_v4();
        let mut chat_id = uuid.as_bytes().to_vec();
        let millis = chrono::Utc::now().timestamp_millis() as u64;
        chat_id.extend_from_slice(&millis.to_be_bytes());
        let chat = service_request::Chat {
            chat_id: chat_id.clone(),
            last_message_id: 0_u64,
            user_name: self.user_name.clone(),
        };
        let resp = chat.new().await;
        if resp.is_err() {
            return Some(
                errors::form_response("CreateOneToOneChat", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::CreateOneToOneChat(
                response::CreateOneToOneChat {
                    status: response::Status::Success as i32,
                    message: None,
                    chat_id: chat_id.clone(),
                },
            )),
        })
    }
}

impl request::CreateGroup {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("CreateGroup", response::Status::BackendError).await,
            );
        }
        let name = self.name.clone();
        if name.len() as u64 > MAX_NAME {
            return Some(
                errors::form_response("CreateGroup", response::Status::BackendError).await,
            );
        }
        if self.users.is_empty() {
            return Some(
                errors::form_response("CreateGroup", response::Status::BackendError).await,
            );
        }
        // let group = service_request::Chat::get(self.)
        //check group with same name is already exists
        let uuid = Uuid::new_v4();
        let mut group_id = uuid.as_bytes().to_vec();
        let millis = chrono::Utc::now().timestamp_millis() as u64;
        group_id.extend_from_slice(&millis.to_be_bytes());
        let group = service_request::Group {
            group_id: group_id.clone(),
            users: self.users.clone(),
            created_by: ctx.user_name.clone(),
            created_at: Utc::now().timestamp() as u64,
            group_name: self.name.clone(),
            last_message_at: 0,
            last_message_id: 0,
        };
        let resp = group.new().await;
        if resp.is_err() {
            return Some(
                errors::form_response("CreateGroup", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::CreateGroup(
                response::CreateGroup {
                    status: response::Status::Success as i32,
                    message: None,
                    chat_id: group_id.clone(),
                },
            )),
        })
    }
}

impl request::UpdateGroup {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("UnFollow", response::Status::BackendError).await);
        }
        todo!()
    }
}

impl request::ListGroups {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("ListGroups", response::Status::BackendError).await);
        }
        if self.user_name.is_empty() {
            return Some(errors::form_response("ListGroups", response::Status::BackendError).await);
        }
        let groups = service_request::Group::list(self.user_name.clone()).await;
        if groups.is_err() {
            return Some(errors::form_response("ListGroups", response::Status::BackendError).await);
        }
        let groups = groups.unwrap();
        let mut result = Vec::new();
        for group in groups {
            result.push(response::Group {
                group_id: group.group_id.clone(),
                users: group.users.clone(),
                created_by: group.created_by.clone(),
                created_at: group.created_at,
                group_name: group.group_name.clone(),
                last_message_at: group.last_message_at,
                last_message_id: group.last_message_id,
            });
        }
        Some(response::Response {
            operation: Some(response::response::Operation::ListGroups(
                response::ListGroups {
                    status: response::Status::Success as i32,
                    message: None,
                    groups: result,
                },
            )),
        })
    }
}

impl request::AddUserToGroup {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("AddUserToGroup", response::Status::BackendError).await,
            );
        }
        let group = service_request::Group::get(self.chat_id.clone()).await;
        if group.is_err() {
            return Some(
                errors::form_response("AddUserToGroup", response::Status::BackendError).await,
            );
        }
        let mut group = group.unwrap();
        group.users.push(self.user_name.clone());
        let resp = group.add_or_remove_user().await;
        if resp.is_err() {
            return Some(
                errors::form_response("AddUserToGroup", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::AddUserToGroup(
                response::AddUserToGroup {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::RemoveGroup {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("RemoveGroup", response::Status::BackendError).await,
            );
        }
        let group = service_request::Group::get(self.group_id.clone()).await;
        if group.is_err() {
            return Some(
                errors::form_response("RemoveGroup", response::Status::BackendError).await,
            );
        }
        let group = group.unwrap();
        let resp = group.remove().await;
        if resp.is_err() {
            return Some(
                errors::form_response("RemoveGroup", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::RemoveGroup(
                response::RemoveGroup {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::ExitFromGroup {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("ExitFromGroup", response::Status::BackendError).await,
            );
        }
        let group = service_request::Group::get(self.chat_id.clone()).await;
        if group.is_err() {
            return Some(
                errors::form_response("ExitFromGroup", response::Status::BackendError).await,
            );
        }
        let mut group = group.unwrap();
        let mut users = group.users.clone();
        if !users.contains(&ctx.user_name) {
            return Some(
                errors::form_response("ExitFromGroup", response::Status::BackendError).await,
            );
        }
        for i in 0..users.len() {
            if users[i] == ctx.user_name.clone() {
                users.remove(i);
            }
        }
        group.users = users;
        let resp = group.add_or_remove_user().await;
        if resp.is_err() {
            return Some(
                errors::form_response("ExitFromGroup", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::ExitFromGroup(
                response::ExitFromGroup {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::GetChat {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("UnFollow", response::Status::BackendError).await);
        }
        todo!()
    }
}

impl request::RemoveUserFromGroup {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("RemoveUserFromGroup", response::Status::BackendError).await,
            );
        }
        let group = service_request::Group::get(self.chat_id.clone()).await;
        if group.is_err() {
            return Some(
                errors::form_response("RemoveUserFromGroup", response::Status::BackendError).await,
            );
        }
        let mut group = group.unwrap();
        let mut users = group.users.clone();
        if !users.contains(&self.user_name) {
            return Some(
                errors::form_response("RemoveUserFromGroup", response::Status::BackendError).await,
            );
        }
        for i in 0..users.len() {
            if users[i] == self.user_name.clone() {
                users.remove(i);
            }
        }
        group.users = users;
        let resp = group.add_or_remove_user().await;
        if resp.is_err() {
            return Some(
                errors::form_response("RemoveUserFromGroup", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::RemoveUserFromGroup(
                response::RemoveUserFromGroup {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::ClearChat {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("ClearChat", response::Status::BackendError).await);
        }

        let chat = service_request::Chat::get(self.chat_id.clone()).await;
        if chat.is_err() {
            return Some(errors::form_response("ClearChat", response::Status::BackendError).await);
        }
        let chat = chat.unwrap();
        let resp = chat.remove(chat.last_message_id).await;
        if resp.is_err() {
            return Some(errors::form_response("ClearChat", response::Status::BackendError).await);
        }
        Some(response::Response {
            operation: Some(response::response::Operation::ClearChat(
                response::ClearChat {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::SendMessage {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("SendMessage", response::Status::BackendError).await,
            );
        }
        if self.content.is_empty() {
            return Some(
                errors::form_response("SendMessage", response::Status::BackendError).await,
            );
        }
        let message = service_request::Message {
            owner: ctx.user_name.clone(),
            content: self.content.clone(),
            chat_id: self.chat_id.clone(),
            created_at: chrono::Utc::now().timestamp() as u64,
            message_id: 0_u64,
        };
        let resp = message.new().await;
        if resp.is_err() {
            return Some(
                errors::form_response("SendMessage", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::SendMessage(
                response::SendMessage {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::RemoveMessage {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("RemoveMessage", response::Status::BackendError).await,
            );
        }
        let message = service_request::Message::get(self.chat_id.clone(), self.message_id).await;
        if message.is_err() {
            return Some(
                errors::form_response("RemoveMessage", response::Status::BackendError).await,
            );
        }
        let message = message.unwrap();
        if self.message_remove == request::MessageRemove::RemoveForAll as i32
            && message.owner != ctx.user_name.clone()
        {
            return Some(
                errors::form_response("RemoveMessage", response::Status::BackendError).await,
            );
        }
        let mut remove = service_request::MessageRemove::DeleteForMe;
        if self.message_remove == request::MessageRemove::RemoveForAll as i32 {
            remove = service_request::MessageRemove::DeleteForEveryOne;
        }
        let resp = message.remove(remove).await;
        if resp.is_err() {
            return Some(
                errors::form_response("RemoveMessage", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::RemoveMessage(
                response::RemoveMessage {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::EditMessage {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("EditMessage", response::Status::BackendError).await,
            );
        }
        let message = service_request::Message::get(self.chat_id.clone(), self.message_id).await;
        if message.is_err() {
            return Some(
                errors::form_response("EditMessage", response::Status::BackendError).await,
            );
        }
        let mut message = message.unwrap();
        if message.owner != ctx.user_name.clone() {
            return Some(
                errors::form_response("EditMessage", response::Status::BackendError).await,
            );
        }
        if self.content.is_empty() {
            return Some(
                errors::form_response("EditMessage", response::Status::BackendError).await,
            );
        }
        message.content = self.content.clone();
        let resp = message.update().await;
        if resp.is_err() {
            return Some(
                errors::form_response("EditMessage", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::EditMessage(
                response::EditMessage {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::ListChat {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("UnFollow", response::Status::BackendError).await);
        }
        todo!()
    }
}

impl request::ReadMessage {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("ReadMessage", response::Status::BackendError).await,
            );
        }

        let message = service_request::Message::get(self.chat_id.clone(), self.message_id).await;
        if message.is_err() {
            return Some(
                errors::form_response("ReadMessage", response::Status::BackendError).await,
            );
        }
        let message = message.unwrap();
        if message.owner == ctx.user_name.clone() {
            return Some(
                errors::form_response("ReadMessage", response::Status::BackendError).await,
            );
        }
        let resp = message.read(ctx.user_name.clone()).await;
        if resp.is_err() {
            return Some(
                errors::form_response("ReadMessage", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::ReadMessage(
                response::ReadMessage {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}
