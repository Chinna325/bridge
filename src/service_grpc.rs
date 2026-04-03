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
        pub async fn add_tweet(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::AddTweet>,
        ) -> Result<
            tonic::Response<super::super::service_response::AddTweet>,
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
                "/service_grpc.ServiceServer/AddTweet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_tweet(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::GetTweet>,
        ) -> Result<
            tonic::Response<super::super::service_response::GetTweet>,
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
                "/service_grpc.ServiceServer/GetTweet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_tweet(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::RemoveTweet>,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveTweet>,
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
                "/service_grpc.ServiceServer/RemoveTweet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_tweet(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::UpdateTweet>,
        ) -> Result<
            tonic::Response<super::super::service_response::UpdateTweet>,
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
                "/service_grpc.ServiceServer/UpdateTweet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_tweets(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::ListTweets>,
        ) -> Result<
            tonic::Response<super::super::service_response::ListTweets>,
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
                "/service_grpc.ServiceServer/ListTweets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_like(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::AddLike>,
        ) -> Result<
            tonic::Response<super::super::service_response::AddLike>,
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
                "/service_grpc.ServiceServer/AddLike",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_like(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::RemoveLike>,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveLike>,
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
                "/service_grpc.ServiceServer/RemoveLike",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn adddis_like(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::AdddisLike>,
        ) -> Result<
            tonic::Response<super::super::service_response::AdddisLike>,
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
                "/service_grpc.ServiceServer/AdddisLike",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_dis_like(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::service_request::RemoveDisLike,
            >,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveDisLike>,
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
                "/service_grpc.ServiceServer/RemoveDisLike",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_love(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::AddLove>,
        ) -> Result<
            tonic::Response<super::super::service_response::AddLove>,
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
                "/service_grpc.ServiceServer/AddLove",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_love(
            &mut self,
            request: impl tonic::IntoRequest<super::super::service_request::RemoveLove>,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveLove>,
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
                "/service_grpc.ServiceServer/RemoveLove",
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
        async fn add_tweet(
            &self,
            request: tonic::Request<super::super::service_request::AddTweet>,
        ) -> Result<
            tonic::Response<super::super::service_response::AddTweet>,
            tonic::Status,
        >;
        async fn get_tweet(
            &self,
            request: tonic::Request<super::super::service_request::GetTweet>,
        ) -> Result<
            tonic::Response<super::super::service_response::GetTweet>,
            tonic::Status,
        >;
        async fn remove_tweet(
            &self,
            request: tonic::Request<super::super::service_request::RemoveTweet>,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveTweet>,
            tonic::Status,
        >;
        async fn update_tweet(
            &self,
            request: tonic::Request<super::super::service_request::UpdateTweet>,
        ) -> Result<
            tonic::Response<super::super::service_response::UpdateTweet>,
            tonic::Status,
        >;
        async fn list_tweets(
            &self,
            request: tonic::Request<super::super::service_request::ListTweets>,
        ) -> Result<
            tonic::Response<super::super::service_response::ListTweets>,
            tonic::Status,
        >;
        async fn add_like(
            &self,
            request: tonic::Request<super::super::service_request::AddLike>,
        ) -> Result<
            tonic::Response<super::super::service_response::AddLike>,
            tonic::Status,
        >;
        async fn remove_like(
            &self,
            request: tonic::Request<super::super::service_request::RemoveLike>,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveLike>,
            tonic::Status,
        >;
        async fn adddis_like(
            &self,
            request: tonic::Request<super::super::service_request::AdddisLike>,
        ) -> Result<
            tonic::Response<super::super::service_response::AdddisLike>,
            tonic::Status,
        >;
        async fn remove_dis_like(
            &self,
            request: tonic::Request<super::super::service_request::RemoveDisLike>,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveDisLike>,
            tonic::Status,
        >;
        async fn add_love(
            &self,
            request: tonic::Request<super::super::service_request::AddLove>,
        ) -> Result<
            tonic::Response<super::super::service_response::AddLove>,
            tonic::Status,
        >;
        async fn remove_love(
            &self,
            request: tonic::Request<super::super::service_request::RemoveLove>,
        ) -> Result<
            tonic::Response<super::super::service_response::RemoveLove>,
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
                "/service_grpc.ServiceServer/AddTweet" => {
                    #[allow(non_camel_case_types)]
                    struct AddTweetSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::AddTweet,
                    > for AddTweetSvc<T> {
                        type Response = super::super::service_response::AddTweet;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::AddTweet,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_tweet(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddTweetSvc(inner);
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
                "/service_grpc.ServiceServer/GetTweet" => {
                    #[allow(non_camel_case_types)]
                    struct GetTweetSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::GetTweet,
                    > for GetTweetSvc<T> {
                        type Response = super::super::service_response::GetTweet;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::GetTweet,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_tweet(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTweetSvc(inner);
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
                "/service_grpc.ServiceServer/RemoveTweet" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveTweetSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::RemoveTweet,
                    > for RemoveTweetSvc<T> {
                        type Response = super::super::service_response::RemoveTweet;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::RemoveTweet,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).remove_tweet(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveTweetSvc(inner);
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
                "/service_grpc.ServiceServer/UpdateTweet" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTweetSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::UpdateTweet,
                    > for UpdateTweetSvc<T> {
                        type Response = super::super::service_response::UpdateTweet;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::UpdateTweet,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_tweet(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateTweetSvc(inner);
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
                "/service_grpc.ServiceServer/ListTweets" => {
                    #[allow(non_camel_case_types)]
                    struct ListTweetsSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::ListTweets,
                    > for ListTweetsSvc<T> {
                        type Response = super::super::service_response::ListTweets;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::ListTweets,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_tweets(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListTweetsSvc(inner);
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
                "/service_grpc.ServiceServer/AddLike" => {
                    #[allow(non_camel_case_types)]
                    struct AddLikeSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<super::super::service_request::AddLike>
                    for AddLikeSvc<T> {
                        type Response = super::super::service_response::AddLike;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::AddLike,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_like(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddLikeSvc(inner);
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
                "/service_grpc.ServiceServer/RemoveLike" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveLikeSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::RemoveLike,
                    > for RemoveLikeSvc<T> {
                        type Response = super::super::service_response::RemoveLike;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::RemoveLike,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_like(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveLikeSvc(inner);
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
                "/service_grpc.ServiceServer/AdddisLike" => {
                    #[allow(non_camel_case_types)]
                    struct AdddisLikeSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::AdddisLike,
                    > for AdddisLikeSvc<T> {
                        type Response = super::super::service_response::AdddisLike;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::AdddisLike,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).adddis_like(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AdddisLikeSvc(inner);
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
                "/service_grpc.ServiceServer/RemoveDisLike" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveDisLikeSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::RemoveDisLike,
                    > for RemoveDisLikeSvc<T> {
                        type Response = super::super::service_response::RemoveDisLike;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::RemoveDisLike,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).remove_dis_like(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveDisLikeSvc(inner);
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
                "/service_grpc.ServiceServer/AddLove" => {
                    #[allow(non_camel_case_types)]
                    struct AddLoveSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<super::super::service_request::AddLove>
                    for AddLoveSvc<T> {
                        type Response = super::super::service_response::AddLove;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::AddLove,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_love(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddLoveSvc(inner);
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
                "/service_grpc.ServiceServer/RemoveLove" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveLoveSvc<T: ServiceServer>(pub Arc<T>);
                    impl<
                        T: ServiceServer,
                    > tonic::server::UnaryService<
                        super::super::service_request::RemoveLove,
                    > for RemoveLoveSvc<T> {
                        type Response = super::super::service_response::RemoveLove;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::service_request::RemoveLove,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_love(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveLoveSvc(inner);
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
