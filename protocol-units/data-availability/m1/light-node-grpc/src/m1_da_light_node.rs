// This file is @generated by prost-build.
/// Request and response messages
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamReadRequest {
    #[prost(string, tag = "1")]
    pub start_height: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlobRequest {
    #[prost(string, tag = "1")]
    pub blob_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlobResponse {
    #[prost(string, tag = "1")]
    pub blob_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "3")]
    pub verified: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlobWriteRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAtHeightRequest {
    #[prost(string, tag = "1")]
    pub height: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchReadRequest {
    #[prost(string, repeated, tag = "1")]
    pub blob_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchReadResponse {
    #[prost(message, repeated, tag = "1")]
    pub blobs: ::prost::alloc::vec::Vec<BlobResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchWriteRequest {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchWriteResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerificationParametersRequest {
    #[prost(enumeration = "VerificationMode", tag = "1")]
    pub mode: i32,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag = "3")]
    pub m: u32,
    #[prost(uint32, tag = "4")]
    pub n: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VerificationMode {
    Cowboy = 0,
    ValidatorIn = 1,
    MOfN = 2,
}
impl VerificationMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VerificationMode::Cowboy => "COWBOY",
            VerificationMode::ValidatorIn => "VALIDATOR_IN",
            VerificationMode::MOfN => "M_OF_N",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COWBOY" => Some(Self::Cowboy),
            "VALIDATOR_IN" => Some(Self::ValidatorIn),
            "M_OF_N" => Some(Self::MOfN),
            _ => None,
        }
    }
}
/// Generated server implementations.
pub mod light_node_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with LightNodeServer.
    #[async_trait]
    pub trait LightNode: Send + Sync + 'static {
        /// Server streaming response type for the StreamReadFromHeight method.
        type StreamReadFromHeightStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::BlobResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Stream blobs from a specified height or from the latest height.
        async fn stream_read_from_height(
            &self,
            request: tonic::Request<super::StreamReadRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamReadFromHeightStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the StreamReadLatest method.
        type StreamReadLatestStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::BlobResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn stream_read_latest(
            &self,
            request: tonic::Request<super::Empty>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamReadLatestStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the StreamWriteBlob method.
        type StreamWriteBlobStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::WriteResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Stream blobs out, either individually or in batches.
        async fn stream_write_blob(
            &self,
            request: tonic::Request<tonic::Streaming<super::BlobWriteRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamWriteBlobStream>,
            tonic::Status,
        >;
        /// Read blobs at a specified height.
        async fn read_at_height(
            &self,
            request: tonic::Request<super::ReadAtHeightRequest>,
        ) -> std::result::Result<tonic::Response<super::BlobResponse>, tonic::Status>;
        /// Batch read and write operations for efficiency.
        async fn batch_read(
            &self,
            request: tonic::Request<super::BatchReadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchReadResponse>,
            tonic::Status,
        >;
        async fn batch_write(
            &self,
            request: tonic::Request<super::BatchWriteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchWriteResponse>,
            tonic::Status,
        >;
        /// Update and manage verification parameters.
        async fn update_verification_parameters(
            &self,
            request: tonic::Request<super::VerificationParametersRequest>,
        ) -> std::result::Result<tonic::Response<super::UpdateResponse>, tonic::Status>;
    }
    /// LightNode service definition
    #[derive(Debug)]
    pub struct LightNodeServer<T: LightNode> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: LightNode> LightNodeServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
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
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for LightNodeServer<T>
    where
        T: LightNode,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/m1_da_light_node.LightNode/StreamReadFromHeight" => {
                    #[allow(non_camel_case_types)]
                    struct StreamReadFromHeightSvc<T: LightNode>(pub Arc<T>);
                    impl<
                        T: LightNode,
                    > tonic::server::ServerStreamingService<super::StreamReadRequest>
                    for StreamReadFromHeightSvc<T> {
                        type Response = super::BlobResponse;
                        type ResponseStream = T::StreamReadFromHeightStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamReadRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LightNode>::stream_read_from_height(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamReadFromHeightSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/m1_da_light_node.LightNode/StreamReadLatest" => {
                    #[allow(non_camel_case_types)]
                    struct StreamReadLatestSvc<T: LightNode>(pub Arc<T>);
                    impl<
                        T: LightNode,
                    > tonic::server::ServerStreamingService<super::Empty>
                    for StreamReadLatestSvc<T> {
                        type Response = super::BlobResponse;
                        type ResponseStream = T::StreamReadLatestStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Empty>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LightNode>::stream_read_latest(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamReadLatestSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/m1_da_light_node.LightNode/StreamWriteBlob" => {
                    #[allow(non_camel_case_types)]
                    struct StreamWriteBlobSvc<T: LightNode>(pub Arc<T>);
                    impl<
                        T: LightNode,
                    > tonic::server::StreamingService<super::BlobWriteRequest>
                    for StreamWriteBlobSvc<T> {
                        type Response = super::WriteResponse;
                        type ResponseStream = T::StreamWriteBlobStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::BlobWriteRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LightNode>::stream_write_blob(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamWriteBlobSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/m1_da_light_node.LightNode/ReadAtHeight" => {
                    #[allow(non_camel_case_types)]
                    struct ReadAtHeightSvc<T: LightNode>(pub Arc<T>);
                    impl<
                        T: LightNode,
                    > tonic::server::UnaryService<super::ReadAtHeightRequest>
                    for ReadAtHeightSvc<T> {
                        type Response = super::BlobResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadAtHeightRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LightNode>::read_at_height(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadAtHeightSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/m1_da_light_node.LightNode/BatchRead" => {
                    #[allow(non_camel_case_types)]
                    struct BatchReadSvc<T: LightNode>(pub Arc<T>);
                    impl<
                        T: LightNode,
                    > tonic::server::UnaryService<super::BatchReadRequest>
                    for BatchReadSvc<T> {
                        type Response = super::BatchReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchReadRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LightNode>::batch_read(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BatchReadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/m1_da_light_node.LightNode/BatchWrite" => {
                    #[allow(non_camel_case_types)]
                    struct BatchWriteSvc<T: LightNode>(pub Arc<T>);
                    impl<
                        T: LightNode,
                    > tonic::server::UnaryService<super::BatchWriteRequest>
                    for BatchWriteSvc<T> {
                        type Response = super::BatchWriteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchWriteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LightNode>::batch_write(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BatchWriteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/m1_da_light_node.LightNode/UpdateVerificationParameters" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateVerificationParametersSvc<T: LightNode>(pub Arc<T>);
                    impl<
                        T: LightNode,
                    > tonic::server::UnaryService<super::VerificationParametersRequest>
                    for UpdateVerificationParametersSvc<T> {
                        type Response = super::UpdateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VerificationParametersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LightNode>::update_verification_parameters(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateVerificationParametersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
    impl<T: LightNode> Clone for LightNodeServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: LightNode> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: LightNode> tonic::server::NamedService for LightNodeServer<T> {
        const NAME: &'static str = "m1_da_light_node.LightNode";
    }
}