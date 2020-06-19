#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Book {
    #[prost(string, tag = "1")]
    pub title: std::string::String,
    #[prost(string, tag = "2")]
    pub author: std::string::String,
}
pub mod book {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Genre {
        Fantasy = 0,
        Crime = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DefaultResponse {
    #[prost(oneof = "default_response::Reply", tags = "1, 2")]
    pub reply: ::std::option::Option<default_response::Reply>,
}
pub mod default_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Reply {
        #[prost(bool, tag = "1")]
        Ok(bool),
        #[prost(string, tag = "2")]
        Error(std::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BookRequest {
    #[prost(int64, tag = "1")]
    pub offset: i64,
    #[prost(int64, tag = "2")]
    pub limit: i64,
}
#[doc = r" Generated client implementations."]
pub mod book_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct BookServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BookServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BookServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn create_book(
            &mut self,
            request: impl tonic::IntoRequest<super::Book>,
        ) -> Result<tonic::Response<super::DefaultResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/book_service.BookService/CreateBook");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_book(
            &mut self,
            request: impl tonic::IntoRequest<super::BookRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Book>>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/book_service.BookService/GetBook");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for BookServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod book_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with BookServiceServer."]
    #[async_trait]
    pub trait BookService: Send + Sync + 'static {
        async fn create_book(
            &self,
            request: tonic::Request<super::Book>,
        ) -> Result<tonic::Response<super::DefaultResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the GetBook method."]
        type GetBookStream: Stream<Item = Result<super::Book, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn get_book(
            &self,
            request: tonic::Request<super::BookRequest>,
        ) -> Result<tonic::Response<Self::GetBookStream>, tonic::Status>;
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct BookServiceServer<T: BookService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: BookService> BookServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T: BookService> Service<http::Request<HyperBody>> for BookServiceServer<T> {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<HyperBody>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/book_service.BookService/CreateBook" => {
                    struct CreateBookSvc<T: BookService>(pub Arc<T>);
                    impl<T: BookService> tonic::server::UnaryService<super::Book> for CreateBookSvc<T> {
                        type Response = super::DefaultResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Book>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_book(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateBookSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/book_service.BookService/GetBook" => {
                    struct GetBookSvc<T: BookService>(pub Arc<T>);
                    impl<T: BookService> tonic::server::ServerStreamingService<super::BookRequest> for GetBookSvc<T> {
                        type Response = super::Book;
                        type ResponseStream = T::GetBookStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BookRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_book(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = GetBookSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: BookService> Clone for BookServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: BookService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BookService> tonic::transport::NamedService for BookServiceServer<T> {
        const NAME: &'static str = "book_service.BookService";
    }
}
