// use std::fs;
use crate::service_response::{self, ServiceResponse};
use crate::{service_grpc, service_request};
use service_grpc::service_server_client::ServiceServerClient;
use service_request::ServiceRequest;
use tonic::transport::Channel;
pub async fn ceate_grpc_connection() -> ServiceServerClient<Channel> {
    service_grpc::service_server_client::ServiceServerClient::connect("127.0.0.1:6678")
        .await
        .unwrap()
}

impl ServiceRequest {
    pub async fn execute(
        &self,
        mut client: ServiceServerClient<Channel>,
    ) -> Option<ServiceResponse> {
        match self.operation.clone() {
            Some(service_request::service_request::Operation::AddUser(req)) => {
                let request = tonic::Request::new(req);
                match client.add_user(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::AddUser(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::AddToDb(req)) => {
                let request = tonic::Request::new(req);
                match client.add_to_db(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::AddToDb(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::CacheItem(req)) => {
                let request = tonic::Request::new(req);
                match client.cache_item(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::CacheItem(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::ClearCache(req)) => {
                let request = tonic::Request::new(req);
                match client.clear_cache(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::ClearCache(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::GetCacheItem(req)) => {
                let request = tonic::Request::new(req);
                match client.get_cache_item(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::GetCacheItem(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }
            _ => {
                panic!("Invalid request");
            }
        }
    }
}
