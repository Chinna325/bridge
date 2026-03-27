use crate::{backend, service_request, service_response};

pub struct Cache {}

impl Cache {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn set(&self, key: Vec<u8>, data: Vec<u8>) -> Option<()> {
        let req = service_request::ServiceRequest {
            operation: Some(service_request::service_request::Operation::CacheItem(
                service_request::CacheItem { key, data },
            )),
        };
        let client = backend::ceate_grpc_connection().await;
        let resp = req.execute(client).await?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::CacheItem(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return None;
            }
        }
        Some(())
    }

    pub async fn get(&self, key: Vec<u8>) -> Option<Vec<u8>> {
        let req = service_request::ServiceRequest {
            operation: Some(service_request::service_request::Operation::GetCacheItem(
                service_request::GetCacheItem { key },
            )),
        };
        let client = backend::ceate_grpc_connection().await;
        let resp = req.execute(client).await?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::GetCacheItem(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return None;
            }
            return Some(resp.data);
        }
        None
    }

    pub async fn clear(&self, key: Vec<u8>) -> Option<()> {
        let req = service_request::ServiceRequest {
            operation: Some(service_request::service_request::Operation::ClearCache(
                service_request::ClearCache { key },
            )),
        };
        let client = backend::ceate_grpc_connection().await;
        let resp = req.execute(client).await?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::ClearCache(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return None;
            }

        }
        Some(())
    }
}
