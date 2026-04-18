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
            if resp.status != response::Status::Success as i32 {
                return None;
            }
            return Some(resp.user?);
        }
        None
    }
    pub async fn update(&mut self) -> Option<()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::UpdateUser(
                service_request::UpdateUser {
                    user: Some(service_request::User {
                        email: self.email.clone(),
                        user_name: self.user_name.clone(),
                        password: self.password.clone(),
                        followers: Vec::new(),
                        profile_picture: Vec::new(),
                    }),
                },
            )),
        };
        let client = backend::ceate_grpc_connection().await;
        let resp = req.execute(client).await?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::UpdateUser(resp)),
        } = resp
        {
            if resp.status != response::Status::Success as i32 {
                return None;
            }
        }
        Some(())
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
        Some(())
    }

    pub async fn set_profile_picture(&mut self, blob: Vec<u8>) -> Option<()> {
        let req = ServiceRequest {
            operation: Some(
                service_request::service_request::Operation::SetProfilePicture(
                    service_request::SetProfilePicture {
                        user_name: self.user_name.clone(),
                        blob_name: self.profile_picture.clone(),
                        data: blob,
                    },
                ),
            ),
        };
        let client = backend::ceate_grpc_connection().await;
        let resp = req.execute(client).await?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::SetProfilePicture(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return None;
            }
        }
        Some(())
    }
    pub async fn remove_profile_picture(&self) -> Option<()> {
        let req = ServiceRequest {
            operation: Some(
                service_request::service_request::Operation::RemoveProfilePicture(
                    service_request::RemoveProfilePicture {
                        user_name: self.user_name.clone(),
                        blob_name: self.profile_picture.clone(),
                    },
                ),
            ),
        };
        let client = backend::ceate_grpc_connection().await;
        let resp = req.execute(client).await?;
        if let service_response::ServiceResponse {
            operation:
                Some(service_response::service_response::Operation::RemoveProfilePicture(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return None;
            }
        }
        Some(())
    }
    pub async fn get_profile_picture(&self) -> Option<Vec<u8>> {
        let req = ServiceRequest {
            operation: Some(
                service_request::service_request::Operation::GetProfilePicture(
                    service_request::GetProfilePicture {
                        blob_name: self.profile_picture.clone(),
                    },
                ),
            ),
        };
        let client = backend::ceate_grpc_connection().await;
        let resp = req.execute(client).await?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::GetProfilePicture(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Some(resp.blob);
            }
        }
        None
    }
}
