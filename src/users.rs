use crate::{
    backend, response,
    service_request::{self, ServiceRequest},
    service_response::{self, User},
};
impl User {
    pub async fn new(email: String, password: Vec<u8>, user_name: String) -> Option<()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::AddToDb(
                service_request::AddToDb {
                    email,
                    user_name,
                    password_hash: password.clone(),
                },
            )),
        };
        let client = backend::ceate_grpc_connection().await;
        let resp = req.execute(client).await?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::AddToDb(resp)),
        } = resp
        {
            if resp.status != response::Status::BackendError as i32 {
                return None;
            }
        }
        Some(())
    }
    pub async fn get(user_name: String) -> Option<Self> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::GetUser(
                service_request::GetUser { user_name },
            )),
        };
        let client = backend::ceate_grpc_connection().await;
        let resp = req.execute(client).await?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::GetUser(resp)),
        } = resp
        {
            if resp.status != response::Status::BackendError as i32 {
                return None;
            }
            Some(resp.user);
        }
        None
    }
    pub async fn update() -> Option<()> {
        todo!()
    }
    pub async fn remove(&self) -> Option<()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::RemoveUser(
                service_request::RemoveUser {
                    user_name: self.user_name.clone(),
                },
            )),
        };
        let client = backend::ceate_grpc_connection().await;
        let resp = req.execute(client).await?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::RemoveUser(resp)),
        } = resp
        {
            if resp.status != response::Status::Success as i32 {
                return None;
            }
        }
        // todo!()
        Some(())
    }
    // pub
}
