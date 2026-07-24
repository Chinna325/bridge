/// Generated client implementations.
pub mod service_server_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ServiceServerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ServiceServerClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ServiceServerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ServiceServerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ServiceServerClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn add_user(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::AddUser>,
        ) -> Result<
            tonic::Response<super::super::service_response::AddUser>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/AddUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn cache_item(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::CacheItem>,
        ) -> Result<
            tonic::Response<super::super::service_response::CacheItem>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/CacheItem",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_cache_item(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::GetCacheItem>,
        ) -> Result<
            tonic::Response<super::super::service_response::GetCacheItem>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/GetCacheItem",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn clear_cache(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::ClearCache>,
        ) -> Result<
            tonic::Response<super::super::service_response::ClearCache>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/ClearCache",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_to_db(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::AddToDb>,
        ) -> Result<
            tonic::Response<super::super::service_response::AddToDb>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/AddToDb",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_user(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::GetUser>,
        ) -> Result<
            tonic::Response<super::super::service_response::GetUser>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/GetUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_user(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::RemoveUser>,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveUser>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/RemoveUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_user(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::UpdateUser>,
        ) -> Result<
            tonic::Response<super::super::service_response::UpdateUser>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/UpdateUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_profile_picture(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::service_request::SetProfilePicture,
            >,
        ) -> Result<
            tonic::Response<super::super::service_response::SetProfilePicture>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/SetProfilePicture",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_profile_picture(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::service_request::GetProfilePicture,
            >,
        ) -> Result<
            tonic::Response<super::super::service_response::GetProfilePicture>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/GetProfilePicture",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_profile_picture(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::service_request::RemoveProfilePicture,
            >,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveProfilePicture>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/RemoveProfilePicture",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_post(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::AddPost>,
        ) -> Result<
            tonic::Response<super::super::service_response::AddPost>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/AddPost",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_post(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::GetPost>,
        ) -> Result<
            tonic::Response<super::super::service_response::GetPost>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/GetPost",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_post(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::RemovePost>,
        ) -> Result<
            tonic::Response<super::super::service_response::RemovePost>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/RemovePost",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_post(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::UpdatePost>,
        ) -> Result<
            tonic::Response<super::super::service_response::UpdatePost>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/UpdatePost",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_posts(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::ListPosts>,
        ) -> Result<
            tonic::Response<super::super::service_response::ListPosts>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/ListPosts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn post_react(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::PostReact>,
        ) -> Result<
            tonic::Response<super::super::service_response::PostReact>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/PostReact",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_reply(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::AddReply>,
        ) -> Result<
            tonic::Response<super::super::service_response::AddReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/AddReply",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_reply(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::RemoveReply>,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/RemoveReply",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_reply(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::GetReply>,
        ) -> Result<
            tonic::Response<super::super::service_response::GetReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/GetReply",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_replies(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::ListReplies>,
        ) -> Result<
            tonic::Response<super::super::service_response::ListReplies>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/ListReplies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_reply(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::UpdateReply>,
        ) -> Result<
            tonic::Response<super::super::service_response::UpdateReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/UpdateReply",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn send_message(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::SendMessage>,
        ) -> Result<
            tonic::Response<super::super::service_response::SendMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/SendMessage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn clear_conversation(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::service_request::ClearConversation,
            >,
        ) -> Result<
            tonic::Response<super::super::service_response::ClearConversation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/ClearConversation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_conversation(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::service_request::GetConversation,
            >,
        ) -> Result<
            tonic::Response<super::super::service_response::GetConversation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/GetConversation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn exit_from_group(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::service_request::ExitFromGroup,
            >,
        ) -> Result<
            tonic::Response<super::super::service_response::ExitFromGroup>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/ExitFromGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_user_from_group(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::service_request::RemoveUserFromGroup,
            >,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveUserFromGroup>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/RemoveUserFromGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_user_to_group(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::service_request::AddUserToGroup,
            >,
        ) -> Result<
            tonic::Response<super::super::service_response::AddUserToGroup>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/AddUserToGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::ListGroups>,
        ) -> Result<
            tonic::Response<super::super::service_response::ListGroups>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/ListGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_group(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::UpdateGroup>,
        ) -> Result<
            tonic::Response<super::super::service_response::UpdateGroup>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/UpdateGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_group(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::RemoveGroup>,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveGroup>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/RemoveGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_group(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::CreateGroup>,
        ) -> Result<
            tonic::Response<super::super::service_response::CreateGroup>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/CreateGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_one_to_one_conversation(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::service_request::CreateOneToOneConversation,
            >,
        ) -> Result<
            tonic::Response<super::super::service_response::CreateOneToOneConversation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/CreateOneToOneConversation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn edit_message(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::EditMessage>,
        ) -> Result<
            tonic::Response<super::super::service_response::EditMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/EditMessage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_message(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::service_request::RemoveMessage,
            >,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/RemoveMessage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_conversation(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::service_request::ListConversation,
            >,
        ) -> Result<
            tonic::Response<super::super::service_response::ListConversation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/ListConversation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn read_message(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::ReadMessage>,
        ) -> Result<
            tonic::Response<super::super::service_response::ReadMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/ReadMessage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_message(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::GetMessage>,
        ) -> Result<
            tonic::Response<super::super::service_response::GetMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/GetMessage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_group(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::GetGroup>,
        ) -> Result<
            tonic::Response<super::super::service_response::GetGroup>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/GetGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn follow(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::Follow>,
        ) -> Result<
            tonic::Response<super::super::service_response::Follow>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/Follow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn un_follow(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::UnFollow>,
        ) -> Result<
            tonic::Response<super::super::service_response::UnFollow>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/UnFollow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_followers(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::service_request::ListFollowers,
            >,
        ) -> Result<
            tonic::Response<super::super::service_response::ListFollowers>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/ListFollowers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_attachment(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::service_request::AddAttachment,
            >,
        ) -> Result<
            tonic::Response<super::super::service_response::AddAttachment>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/AddAttachment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn read_attachment(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::service_request::ReadAttachment,
            >,
        ) -> Result<
            tonic::Response<super::super::service_response::ReadAttachment>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/ReadAttachment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_attachment(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::service_request::RemoveAttachment,
            >,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveAttachment>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/RemoveAttachment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_attachments(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::service_request::ListAttachments,
            >,
        ) -> Result<
            tonic::Response<super::super::service_response::ListAttachments>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_grpc.ServiceServer/ListAttachments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod service_server_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ServiceServerServer.
    #[async_trait]
    pub trait ServiceServer: Send + Sync + 'static {
        async fn add_user(
            &self,
            request: tonic::Request<super::super::service_request::AddUser>,
        ) -> Result<
            tonic::Response<super::super::service_response::AddUser>,
            tonic::Status,
        >;
        async fn cache_item(
            &self,
            request: tonic::Request<super::super::service_request::CacheItem>,
        ) -> Result<
            tonic::Response<super::super::service_response::CacheItem>,
            tonic::Status,
        >;
        async fn get_cache_item(
            &self,
            request: tonic::Request<super::super::service_request::GetCacheItem>,
        ) -> Result<
            tonic::Response<super::super::service_response::GetCacheItem>,
            tonic::Status,
        >;
        async fn clear_cache(
            &self,
            request: tonic::Request<super::super::service_request::ClearCache>,
        ) -> Result<
            tonic::Response<super::super::service_response::ClearCache>,
            tonic::Status,
        >;
        async fn add_to_db(
            &self,
            request: tonic::Request<super::super::service_request::AddToDb>,
        ) -> Result<
            tonic::Response<super::super::service_response::AddToDb>,
            tonic::Status,
        >;
        async fn get_user(
            &self,
            request: tonic::Request<super::super::service_request::GetUser>,
        ) -> Result<
            tonic::Response<super::super::service_response::GetUser>,
            tonic::Status,
        >;
        async fn remove_user(
            &self,
            request: tonic::Request<super::super::service_request::RemoveUser>,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveUser>,
            tonic::Status,
        >;
        async fn update_user(
            &self,
            request: tonic::Request<super::super::service_request::UpdateUser>,
        ) -> Result<
            tonic::Response<super::super::service_response::UpdateUser>,
            tonic::Status,
        >;
        async fn set_profile_picture(
            &self,
            request: tonic::Request<super::super::service_request::SetProfilePicture>,
        ) -> Result<
            tonic::Response<super::super::service_response::SetProfilePicture>,
            tonic::Status,
        >;
        async fn get_profile_picture(
            &self,
            request: tonic::Request<super::super::service_request::GetProfilePicture>,
        ) -> Result<
            tonic::Response<super::super::service_response::GetProfilePicture>,
            tonic::Status,
        >;
        async fn remove_profile_picture(
            &self,
            request: tonic::Request<super::super::service_request::RemoveProfilePicture>,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveProfilePicture>,
            tonic::Status,
        >;
        async fn add_post(
            &self,
            request: tonic::Request<super::super::service_request::AddPost>,
        ) -> Result<
            tonic::Response<super::super::service_response::AddPost>,
            tonic::Status,
        >;
        async fn get_post(
            &self,
            request: tonic::Request<super::super::service_request::GetPost>,
        ) -> Result<
            tonic::Response<super::super::service_response::GetPost>,
            tonic::Status,
        >;
        async fn remove_post(
            &self,
            request: tonic::Request<super::super::service_request::RemovePost>,
        ) -> Result<
            tonic::Response<super::super::service_response::RemovePost>,
            tonic::Status,
        >;
        async fn update_post(
            &self,
            request: tonic::Request<super::super::service_request::UpdatePost>,
        ) -> Result<
            tonic::Response<super::super::service_response::UpdatePost>,
            tonic::Status,
        >;
        async fn list_posts(
            &self,
            request: tonic::Request<super::super::service_request::ListPosts>,
        ) -> Result<
            tonic::Response<super::super::service_response::ListPosts>,
            tonic::Status,
        >;
        async fn post_react(
            &self,
            request: tonic::Request<super::super::service_request::PostReact>,
        ) -> Result<
            tonic::Response<super::super::service_response::PostReact>,
            tonic::Status,
        >;
        async fn add_reply(
            &self,
            request: tonic::Request<super::super::service_request::AddReply>,
        ) -> Result<
            tonic::Response<super::super::service_response::AddReply>,
            tonic::Status,
        >;
        async fn remove_reply(
            &self,
            request: tonic::Request<super::super::service_request::RemoveReply>,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveReply>,
            tonic::Status,
        >;
        async fn get_reply(
            &self,
            request: tonic::Request<super::super::service_request::GetReply>,
        ) -> Result<
            tonic::Response<super::super::service_response::GetReply>,
            tonic::Status,
        >;
        async fn list_replies(
            &self,
            request: tonic::Request<super::super::service_request::ListReplies>,
        ) -> Result<
            tonic::Response<super::super::service_response::ListReplies>,
            tonic::Status,
        >;
        async fn update_reply(
            &self,
            request: tonic::Request<super::super::service_request::UpdateReply>,
        ) -> Result<
            tonic::Response<super::super::service_response::UpdateReply>,
            tonic::Status,
        >;
        async fn send_message(
            &self,
            request: tonic::Request<super::super::service_request::SendMessage>,
        ) -> Result<
            tonic::Response<super::super::service_response::SendMessage>,
            tonic::Status,
        >;
        async fn clear_conversation(
            &self,
            request: tonic::Request<super::super::service_request::ClearConversation>,
        ) -> Result<
            tonic::Response<super::super::service_response::ClearConversation>,
            tonic::Status,
        >;
        async fn get_conversation(
            &self,
            request: tonic::Request<super::super::service_request::GetConversation>,
        ) -> Result<
            tonic::Response<super::super::service_response::GetConversation>,
            tonic::Status,
        >;
        async fn exit_from_group(
            &self,
            request: tonic::Request<super::super::service_request::ExitFromGroup>,
        ) -> Result<
            tonic::Response<super::super::service_response::ExitFromGroup>,
            tonic::Status,
        >;
        async fn remove_user_from_group(
            &self,
            request: tonic::Request<super::super::service_request::RemoveUserFromGroup>,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveUserFromGroup>,
            tonic::Status,
        >;
        async fn add_user_to_group(
            &self,
            request: tonic::Request<super::super::service_request::AddUserToGroup>,
        ) -> Result<
            tonic::Response<super::super::service_response::AddUserToGroup>,
            tonic::Status,
        >;
        async fn list_groups(
            &self,
            request: tonic::Request<super::super::service_request::ListGroups>,
        ) -> Result<
            tonic::Response<super::super::service_response::ListGroups>,
            tonic::Status,
        >;
        async fn update_group(
            &self,
            request: tonic::Request<super::super::service_request::UpdateGroup>,
        ) -> Result<
            tonic::Response<super::super::service_response::UpdateGroup>,
            tonic::Status,
        >;
        async fn remove_group(
            &self,
            request: tonic::Request<super::super::service_request::RemoveGroup>,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveGroup>,
            tonic::Status,
        >;
        async fn create_group(
            &self,
            request: tonic::Request<super::super::service_request::CreateGroup>,
        ) -> Result<
            tonic::Response<super::super::service_response::CreateGroup>,
            tonic::Status,
        >;
        async fn create_one_to_one_conversation(
            &self,
            request: tonic::Request<
                super::super::service_request::CreateOneToOneConversation,
            >,
        ) -> Result<
            tonic::Response<super::super::service_response::CreateOneToOneConversation>,
            tonic::Status,
        >;
        async fn edit_message(
            &self,
            request: tonic::Request<super::super::service_request::EditMessage>,
        ) -> Result<
            tonic::Response<super::super::service_response::EditMessage>,
            tonic::Status,
        >;
        async fn remove_message(
            &self,
            request: tonic::Request<super::super::service_request::RemoveMessage>,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveMessage>,
            tonic::Status,
        >;
        async fn list_conversation(
            &self,
            request: tonic::Request<super::super::service_request::ListConversation>,
        ) -> Result<
            tonic::Response<super::super::service_response::ListConversation>,
            tonic::Status,
        >;
        async fn read_message(
            &self,
            request: tonic::Request<super::super::service_request::ReadMessage>,
        ) -> Result<
            tonic::Response<super::super::service_response::ReadMessage>,
            tonic::Status,
        >;
        async fn get_message(
            &self,
            request: tonic::Request<super::super::service_request::GetMessage>,
        ) -> Result<
            tonic::Response<super::super::service_response::GetMessage>,
            tonic::Status,
        >;
        async fn get_group(
            &self,
            request: tonic::Request<super::super::service_request::GetGroup>,
        ) -> Result<
            tonic::Response<super::super::service_response::GetGroup>,
            tonic::Status,
        >;
        async fn follow(
            &self,
            request: tonic::Request<super::super::service_request::Follow>,
        ) -> Result<
            tonic::Response<super::super::service_response::Follow>,
            tonic::Status,
        >;
        async fn un_follow(
            &self,
            request: tonic::Request<super::super::service_request::UnFollow>,
        ) -> Result<
            tonic::Response<super::super::service_response::UnFollow>,
            tonic::Status,
        >;
        async fn list_followers(
            &self,
            request: tonic::Request<super::super::service_request::ListFollowers>,
        ) -> Result<
            tonic::Response<super::super::service_response::ListFollowers>,
            tonic::Status,
        >;
        async fn add_attachment(
            &self,
            request: tonic::Request<super::super::service_request::AddAttachment>,
        ) -> Result<
            tonic::Response<super::super::service_response::AddAttachment>,
            tonic::Status,
        >;
        async fn read_attachment(
            &self,
            request: tonic::Request<super::super::service_request::ReadAttachment>,
        ) -> Result<
            tonic::Response<super::super::service_response::ReadAttachment>,
            tonic::Status,
        >;
        async fn remove_attachment(
            &self,
            request: tonic::Request<super::super::service_request::RemoveAttachment>,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveAttachment>,
            tonic::Status,
        >;
        async fn list_attachments(
            &self,
            request: tonic::Request<super::super::service_request::ListAttachments>,
        ) -> Result<
            tonic::Response<super::super::service_response::ListAttachments>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ServiceServerServer<T: ServiceServer> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ServiceServer> ServiceServerServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ServiceServerServer<T>
    where
        T: ServiceServer,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/service_grpc.ServiceServer/AddUser" => {
                    #[allow(non_camel_case_types)]
                    struct AddUserSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<super::super::service_request::AddUser>
                    for AddUserSvc<T> {
                        type Response = super::super::service_response::AddUser;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::AddUser,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/CacheItem" => {
                    #[allow(non_camel_case_types)]
                    struct CacheItemSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::CacheItem,
                    > for CacheItemSvc<T> {
                        type Response = super::super::service_response::CacheItem;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::CacheItem,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).cache_item(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CacheItemSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/GetCacheItem" => {
                    #[allow(non_camel_case_types)]
                    struct GetCacheItemSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::GetCacheItem,
                    > for GetCacheItemSvc<T> {
                        type Response = super::super::service_response::GetCacheItem;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::GetCacheItem,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_cache_item(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCacheItemSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/ClearCache" => {
                    #[allow(non_camel_case_types)]
                    struct ClearCacheSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::ClearCache,
                    > for ClearCacheSvc<T> {
                        type Response = super::super::service_response::ClearCache;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::ClearCache,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).clear_cache(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClearCacheSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/AddToDb" => {
                    #[allow(non_camel_case_types)]
                    struct AddToDbSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<super::super::service_request::AddToDb>
                    for AddToDbSvc<T> {
                        type Response = super::super::service_response::AddToDb;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::AddToDb,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_to_db(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddToDbSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/GetUser" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<super::super::service_request::GetUser>
                    for GetUserSvc<T> {
                        type Response = super::super::service_response::GetUser;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::GetUser,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/RemoveUser" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveUserSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::RemoveUser,
                    > for RemoveUserSvc<T> {
                        type Response = super::super::service_response::RemoveUser;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::RemoveUser,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/UpdateUser" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateUserSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::UpdateUser,
                    > for UpdateUserSvc<T> {
                        type Response = super::super::service_response::UpdateUser;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::UpdateUser,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/SetProfilePicture" => {
                    #[allow(non_camel_case_types)]
                    struct SetProfilePictureSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::SetProfilePicture,
                    > for SetProfilePictureSvc<T> {
                        type Response = super::super::service_response::SetProfilePicture;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::SetProfilePicture,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_profile_picture(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetProfilePictureSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/GetProfilePicture" => {
                    #[allow(non_camel_case_types)]
                    struct GetProfilePictureSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::GetProfilePicture,
                    > for GetProfilePictureSvc<T> {
                        type Response = super::super::service_response::GetProfilePicture;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::GetProfilePicture,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_profile_picture(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProfilePictureSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/RemoveProfilePicture" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveProfilePictureSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::RemoveProfilePicture,
                    > for RemoveProfilePictureSvc<T> {
                        type Response = super::super::service_response::RemoveProfilePicture;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::RemoveProfilePicture,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).remove_profile_picture(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveProfilePictureSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/AddPost" => {
                    #[allow(non_camel_case_types)]
                    struct AddPostSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<super::super::service_request::AddPost>
                    for AddPostSvc<T> {
                        type Response = super::super::service_response::AddPost;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::AddPost,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_post(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddPostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/GetPost" => {
                    #[allow(non_camel_case_types)]
                    struct GetPostSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<super::super::service_request::GetPost>
                    for GetPostSvc<T> {
                        type Response = super::super::service_response::GetPost;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::GetPost,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_post(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/RemovePost" => {
                    #[allow(non_camel_case_types)]
                    struct RemovePostSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::RemovePost,
                    > for RemovePostSvc<T> {
                        type Response = super::super::service_response::RemovePost;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::RemovePost,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_post(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemovePostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/UpdatePost" => {
                    #[allow(non_camel_case_types)]
                    struct UpdatePostSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::UpdatePost,
                    > for UpdatePostSvc<T> {
                        type Response = super::super::service_response::UpdatePost;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::UpdatePost,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_post(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdatePostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/ListPosts" => {
                    #[allow(non_camel_case_types)]
                    struct ListPostsSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::ListPosts,
                    > for ListPostsSvc<T> {
                        type Response = super::super::service_response::ListPosts;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::ListPosts,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_posts(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListPostsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/PostReact" => {
                    #[allow(non_camel_case_types)]
                    struct PostReactSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::PostReact,
                    > for PostReactSvc<T> {
                        type Response = super::super::service_response::PostReact;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::PostReact,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).post_react(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PostReactSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/AddReply" => {
                    #[allow(non_camel_case_types)]
                    struct AddReplySvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::AddReply,
                    > for AddReplySvc<T> {
                        type Response = super::super::service_response::AddReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::AddReply,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_reply(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddReplySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/RemoveReply" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveReplySvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::RemoveReply,
                    > for RemoveReplySvc<T> {
                        type Response = super::super::service_response::RemoveReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::RemoveReply,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).remove_reply(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveReplySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/GetReply" => {
                    #[allow(non_camel_case_types)]
                    struct GetReplySvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::GetReply,
                    > for GetReplySvc<T> {
                        type Response = super::super::service_response::GetReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::GetReply,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_reply(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetReplySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/ListReplies" => {
                    #[allow(non_camel_case_types)]
                    struct ListRepliesSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::ListReplies,
                    > for ListRepliesSvc<T> {
                        type Response = super::super::service_response::ListReplies;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::ListReplies,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_replies(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListRepliesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/UpdateReply" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateReplySvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::UpdateReply,
                    > for UpdateReplySvc<T> {
                        type Response = super::super::service_response::UpdateReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::UpdateReply,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_reply(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateReplySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/SendMessage" => {
                    #[allow(non_camel_case_types)]
                    struct SendMessageSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::SendMessage,
                    > for SendMessageSvc<T> {
                        type Response = super::super::service_response::SendMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::SendMessage,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).send_message(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/ClearConversation" => {
                    #[allow(non_camel_case_types)]
                    struct ClearConversationSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::ClearConversation,
                    > for ClearConversationSvc<T> {
                        type Response = super::super::service_response::ClearConversation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::ClearConversation,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).clear_conversation(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClearConversationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/GetConversation" => {
                    #[allow(non_camel_case_types)]
                    struct GetConversationSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::GetConversation,
                    > for GetConversationSvc<T> {
                        type Response = super::super::service_response::GetConversation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::GetConversation,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_conversation(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetConversationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/ExitFromGroup" => {
                    #[allow(non_camel_case_types)]
                    struct ExitFromGroupSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::ExitFromGroup,
                    > for ExitFromGroupSvc<T> {
                        type Response = super::super::service_response::ExitFromGroup;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::ExitFromGroup,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).exit_from_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExitFromGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/RemoveUserFromGroup" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveUserFromGroupSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::RemoveUserFromGroup,
                    > for RemoveUserFromGroupSvc<T> {
                        type Response = super::super::service_response::RemoveUserFromGroup;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::RemoveUserFromGroup,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).remove_user_from_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveUserFromGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/AddUserToGroup" => {
                    #[allow(non_camel_case_types)]
                    struct AddUserToGroupSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::AddUserToGroup,
                    > for AddUserToGroupSvc<T> {
                        type Response = super::super::service_response::AddUserToGroup;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::AddUserToGroup,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).add_user_to_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddUserToGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/ListGroups" => {
                    #[allow(non_camel_case_types)]
                    struct ListGroupsSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::ListGroups,
                    > for ListGroupsSvc<T> {
                        type Response = super::super::service_response::ListGroups;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::ListGroups,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_groups(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListGroupsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/UpdateGroup" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateGroupSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::UpdateGroup,
                    > for UpdateGroupSvc<T> {
                        type Response = super::super::service_response::UpdateGroup;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::UpdateGroup,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/RemoveGroup" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveGroupSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::RemoveGroup,
                    > for RemoveGroupSvc<T> {
                        type Response = super::super::service_response::RemoveGroup;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::RemoveGroup,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).remove_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/CreateGroup" => {
                    #[allow(non_camel_case_types)]
                    struct CreateGroupSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::CreateGroup,
                    > for CreateGroupSvc<T> {
                        type Response = super::super::service_response::CreateGroup;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::CreateGroup,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/CreateOneToOneConversation" => {
                    #[allow(non_camel_case_types)]
                    struct CreateOneToOneConversationSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::CreateOneToOneConversation,
                    > for CreateOneToOneConversationSvc<T> {
                        type Response = super::super::service_response::CreateOneToOneConversation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::CreateOneToOneConversation,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_one_to_one_conversation(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateOneToOneConversationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/EditMessage" => {
                    #[allow(non_camel_case_types)]
                    struct EditMessageSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::EditMessage,
                    > for EditMessageSvc<T> {
                        type Response = super::super::service_response::EditMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::EditMessage,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).edit_message(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/RemoveMessage" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveMessageSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::RemoveMessage,
                    > for RemoveMessageSvc<T> {
                        type Response = super::super::service_response::RemoveMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::RemoveMessage,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).remove_message(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/ListConversation" => {
                    #[allow(non_camel_case_types)]
                    struct ListConversationSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::ListConversation,
                    > for ListConversationSvc<T> {
                        type Response = super::super::service_response::ListConversation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::ListConversation,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_conversation(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListConversationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/ReadMessage" => {
                    #[allow(non_camel_case_types)]
                    struct ReadMessageSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::ReadMessage,
                    > for ReadMessageSvc<T> {
                        type Response = super::super::service_response::ReadMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::ReadMessage,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).read_message(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/GetMessage" => {
                    #[allow(non_camel_case_types)]
                    struct GetMessageSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::GetMessage,
                    > for GetMessageSvc<T> {
                        type Response = super::super::service_response::GetMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::GetMessage,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_message(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/GetGroup" => {
                    #[allow(non_camel_case_types)]
                    struct GetGroupSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::GetGroup,
                    > for GetGroupSvc<T> {
                        type Response = super::super::service_response::GetGroup;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::GetGroup,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_group(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/Follow" => {
                    #[allow(non_camel_case_types)]
                    struct FollowSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<super::super::service_request::Follow>
                    for FollowSvc<T> {
                        type Response = super::super::service_response::Follow;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::Follow,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).follow(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FollowSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/UnFollow" => {
                    #[allow(non_camel_case_types)]
                    struct UnFollowSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::UnFollow,
                    > for UnFollowSvc<T> {
                        type Response = super::super::service_response::UnFollow;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::UnFollow,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).un_follow(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnFollowSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/ListFollowers" => {
                    #[allow(non_camel_case_types)]
                    struct ListFollowersSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::ListFollowers,
                    > for ListFollowersSvc<T> {
                        type Response = super::super::service_response::ListFollowers;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::ListFollowers,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_followers(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListFollowersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/AddAttachment" => {
                    #[allow(non_camel_case_types)]
                    struct AddAttachmentSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::AddAttachment,
                    > for AddAttachmentSvc<T> {
                        type Response = super::super::service_response::AddAttachment;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::AddAttachment,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).add_attachment(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddAttachmentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/ReadAttachment" => {
                    #[allow(non_camel_case_types)]
                    struct ReadAttachmentSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::ReadAttachment,
                    > for ReadAttachmentSvc<T> {
                        type Response = super::super::service_response::ReadAttachment;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::ReadAttachment,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).read_attachment(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadAttachmentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/RemoveAttachment" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveAttachmentSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::RemoveAttachment,
                    > for RemoveAttachmentSvc<T> {
                        type Response = super::super::service_response::RemoveAttachment;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::RemoveAttachment,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).remove_attachment(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveAttachmentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_grpc.ServiceServer/ListAttachments" => {
                    #[allow(non_camel_case_types)]
                    struct ListAttachmentsSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::ListAttachments,
                    > for ListAttachmentsSvc<T> {
                        type Response = super::super::service_response::ListAttachments;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::ListAttachments,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_attachments(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListAttachmentsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ServiceServer> Clone for ServiceServerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ServiceServer> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ServiceServer> tonic::transport::NamedService for ServiceServerServer<T> {
        const NAME: &'static str = "service_grpc.ServiceServer";
    }
}
