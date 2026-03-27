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
