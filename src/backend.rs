// use std::fs;
use crate::service_response::{self, ServiceResponse};
use crate::{service_grpc, service_request};
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
        // println!("Service Request :{:?}", self.clone());
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

            Some(service_request::service_request::Operation::AddTweet(req)) => {
                let request = tonic::Request::new(req);
                match client.add_tweet(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::AddTweet(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::GetTweet(req)) => {
                let request = tonic::Request::new(req);
                match client.get_tweet(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::GetTweet(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::ListTweets(req)) => {
                let request = tonic::Request::new(req);
                match client.list_tweets(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::ListTweets(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::UpdateTweet(req)) => {
                let request = tonic::Request::new(req);
                match client.update_tweet(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::UpdateTweet(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }

            Some(service_request::service_request::Operation::RemoveTweet(req)) => {
                let request = tonic::Request::new(req);
                match client.remove_tweet(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::RemoveTweet(
                                resp.into_inner(),
                            ),
                        ),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::TweetReact(req)) => {
                let request = tonic::Request::new(req);
                match client.tweet_react(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::TweetReact(
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

            Some(service_request::service_request::Operation::CreateOneToOneChat(req)) => {
                let request = tonic::Request::new(req);
                match client.create_one_to_one_chat(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(
                            service_response::service_response::Operation::CreateOneToOneChat(
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
            Some(service_request::service_request::Operation::GetChat(req)) => {
                let request = tonic::Request::new(req);
                match client.get_chat(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::GetChat(
                            resp.into_inner(),
                        )),
                    }),
                    Err(_) => None,
                }
            }
            Some(service_request::service_request::Operation::ClearChat(req)) => {
                let request = tonic::Request::new(req);
                match client.clear_chat(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::ClearChat(
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
            Some(service_request::service_request::Operation::ListChat(req)) => {
                let request = tonic::Request::new(req);
                match client.list_chat(request).await {
                    Ok(resp) => Some(ServiceResponse {
                        operation: Some(service_response::service_response::Operation::ListChat(
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
            _ => {
                panic!("Invalid request");
            }
        };
        // let re
        let resp = resp.unwrap();
        // println!("Service Response :{:?}", resp);
        Some(resp)
    }
}
