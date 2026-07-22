// use std::fs;
use crate::protos::service_response::{self, ServiceResponse};
use crate::protos::{service_grpc, service_request};
use service_grpc::service_server_client::ServiceServerClient;
use service_request::ServiceRequest;
use tonic::transport::Channel;
pub async fn ceate_grpc_connection() -> ServiceServerClient<Channel> {
    service_grpc::service_server_client::ServiceServerClient::connect("http://127.0.0.1:5576")
        .await
        .unwrap()
}

impl ServiceRequest {
    pub async fn execute(
        &self,
        mut client: ServiceServerClient<Channel>,
    ) -> Option<ServiceResponse> {
        println!("Service Request :{:?}", self.clone());
        let resp = match self.operation.clone() {
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
            Some(service_request::service_request::Operation::SetProfilePicture(req)) => {
                let request = tonic::Request::new(req);
                match client.set_profile_picture(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::SetProfilePicture(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::GetProfilePicture(req)) => {
                let request = tonic::Request::new(req);
                match client.get_profile_picture(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::GetProfilePicture(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::RemoveProfilePicture(req)) => {
                let request = tonic::Request::new(req);
                match client.remove_profile_picture(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::RemoveProfilePicture(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }

            Some(service_request::service_request::Operation::AddPost(req)) => {
                let request = tonic::Request::new(req);
                match client.add_post(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::AddPost(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::GetPost(req)) => {
                let request = tonic::Request::new(req);
                match client.get_post(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::GetPost(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::ListPosts(req)) => {
                let request = tonic::Request::new(req);
                match client.list_posts(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::ListPosts(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::UpdatePost(req)) => {
                let request = tonic::Request::new(req);
                match client.update_post(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::UpdatePost(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }

            Some(service_request::service_request::Operation::RemovePost(req)) => {
                let request = tonic::Request::new(req);
                match client.remove_post(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::RemovePost(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::PostReact(req)) => {
                let request = tonic::Request::new(req);
                match client.post_react(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::PostReact(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::AddReply(req)) => {
                let request = tonic::Request::new(req);
                match client.add_reply(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::AddReply(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::UpdateReply(req)) => {
                let request = tonic::Request::new(req);
                match client.update_reply(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::UpdateReply(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }

            Some(service_request::service_request::Operation::RemoveReply(req)) => {
                let request = tonic::Request::new(req);
                match client.remove_reply(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::RemoveReply(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::GetReply(req)) => {
                let request = tonic::Request::new(req);
                match client.get_reply(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::GetReply(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::ListReplies(req)) => {
                let request = tonic::Request::new(req);
                match client.list_replies(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::ListReplies(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }

            Some(service_request::service_request::Operation::CreateOneToOneConversation(req)) => {
                let request = tonic::Request::new(req);
                match client.create_one_to_one_conversation(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::CreateOneToOneConversation(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::CreateGroup(req)) => {
                let request = tonic::Request::new(req);
                match client.create_group(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::CreateGroup(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::RemoveGroup(req)) => {
                let request = tonic::Request::new(req);
                match client.remove_group(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::RemoveGroup(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::UpdateGroup(req)) => {
                let request = tonic::Request::new(req);
                match client.update_group(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::UpdateGroup(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::ListGroups(req)) => {
                let request = tonic::Request::new(req);
                match client.list_groups(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::ListGroups(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::AddUserToGroup(req)) => {
                let request = tonic::Request::new(req);
                match client.add_user_to_group(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::AddUserToGroup(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::RemoveUserFromGroup(req)) => {
                let request = tonic::Request::new(req);
                match client.remove_user_from_group(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::RemoveUserFromGroup(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::ExitFromGroup(req)) => {
                let request = tonic::Request::new(req);
                match client.exit_from_group(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::ExitFromGroup(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::GetConversation(req)) => {
                let request = tonic::Request::new(req);
                match client.get_conversation(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::GetConversation(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::ClearConversation(req)) => {
                let request = tonic::Request::new(req);
                match client.clear_conversation(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::ClearConversation(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }

            Some(service_request::service_request::Operation::SendMessage(req)) => {
                let request = tonic::Request::new(req);
                match client.send_message(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::SendMessage(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::RemoveMessage(req)) => {
                let request = tonic::Request::new(req);
                match client.remove_message(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::RemoveMessage(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::EditMessage(req)) => {
                let request = tonic::Request::new(req);
                match client.edit_message(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::EditMessage(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::ListConversation(req)) => {
                let request = tonic::Request::new(req);
                match client.list_conversation(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::ListConversation(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }

            Some(service_request::service_request::Operation::ReadMessage(req)) => {
                let request = tonic::Request::new(req);
                match client.read_message(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::ReadMessage(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::GetMessage(req)) => {
                let request = tonic::Request::new(req);
                match client.get_message(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::GetMessage(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::GetGroup(req)) => {
                let request = tonic::Request::new(req);
                match client.get_group(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::GetGroup(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }

            Some(service_request::service_request::Operation::GetUser(req)) => {
                let request = tonic::Request::new(req);
                match client.get_user(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::GetUser(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }

            Some(service_request::service_request::Operation::Follow(req)) => {
                let request = tonic::Request::new(req);
                match client.follow(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::Follow(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::UnFollow(req)) => {
                let request = tonic::Request::new(req);
                match client.un_follow(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::UnFollow(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }

            Some(service_request::service_request::Operation::ListFollowers(req)) => {
                let request = tonic::Request::new(req);
                match client.list_followers(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::ListFollowers(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }
            _ => {
                panic!("Invalid request :{:?}", &self.operation);
            }
        };
        // let re
        let resp = resp.unwrap();
        println!("Service Response :{:?}", resp);
        Some(resp)
    }
}
