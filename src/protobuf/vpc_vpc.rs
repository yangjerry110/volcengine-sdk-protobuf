#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeVpcsReq {
    #[prost(string, repeated, tag = "1")]
    pub vpc_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub vpc_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub project_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "4")]
    pub is_default: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "5")]
    pub vpc_owner_id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "6")]
    pub page_number: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "7")]
    pub page_size: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "8")]
    pub next_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "9")]
    pub max_results: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "10")]
    pub tag_filters: ::prost::alloc::vec::Vec<DescribeVpcsTagFilterReq>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeVpcsTagFilterReq {
    #[prost(string, optional, tag = "1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeVpcsResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<DescribeVpcsResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeVpcsResultResp {
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub total_count: i64,
    #[prost(int64, tag = "3")]
    pub page_number: i64,
    #[prost(int64, tag = "4")]
    pub page_size: i64,
    #[prost(string, optional, tag = "5")]
    pub next_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "6")]
    pub vpcs: ::prost::alloc::vec::Vec<DescribeVpcsResultVpcResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeVpcsResultVpcResp {
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub vpc_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub vpc_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub cidr_block: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub ipv6_cidr_block: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub network_acl_num: ::prost::alloc::string::String,
    #[prost(bool, tag = "8")]
    pub is_default: bool,
    #[prost(string, tag = "9")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub project_name: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub creation_time: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub update_time: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "13")]
    #[serde(default)]
    pub secondary_cidr_blocks: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "14")]
    #[serde(default)]
    pub user_cidr_blocks: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "15")]
    pub subnet_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "16")]
    pub route_table_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "17")]
    pub security_group_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "18")]
    pub nat_gateway_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "19")]
    #[serde(default)]
    pub dns_servers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "20")]
    pub associate_cens: ::prost::alloc::vec::Vec<DescribeVpcsResultVpcAssociateCenResp>,
    #[prost(message, repeated, tag = "21")]
    #[serde(default)]
    pub tags: ::prost::alloc::vec::Vec<DescribeVpcsResultVpcTagResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeVpcsResultVpcAssociateCenResp {
    #[prost(string, optional, tag = "1")]
    pub cen_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub cen_owner_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub cen_status: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeVpcsResultVpcTagResp {
    #[prost(string, optional, tag = "1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMetadata {
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub action: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub service: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub region: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub error: ::core::option::Option<ResponseMetadataErr>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMetadataErr {
    #[prost(int64, optional, tag = "1")]
    pub code_n: ::core::option::Option<i64>,
    #[prost(string, tag = "2")]
    pub code: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod vpc_vpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct VpcVpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VpcVpcClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> VpcVpcClient<T>
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
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> VpcVpcClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            VpcVpcClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// DescribeVpcs
        pub async fn describe_vpcs(
            &mut self,
            request: impl tonic::IntoRequest<super::DescribeVpcsReq>,
        ) -> std::result::Result<tonic::Response<super::DescribeVpcsResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vpc_vpc.VpcVpc/DescribeVpcs");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vpc_vpc.VpcVpc", "DescribeVpcs"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod vpc_vpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with VpcVpcServer.
    #[async_trait]
    pub trait VpcVpc: Send + Sync + 'static {
        /// DescribeVpcs
        async fn describe_vpcs(
            &self,
            request: tonic::Request<super::DescribeVpcsReq>,
        ) -> std::result::Result<tonic::Response<super::DescribeVpcsResp>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct VpcVpcServer<T: VpcVpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: VpcVpc> VpcVpcServer<T> {
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for VpcVpcServer<T>
    where
        T: VpcVpc,
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
                "/vpc_vpc.VpcVpc/DescribeVpcs" => {
                    #[allow(non_camel_case_types)]
                    struct DescribeVpcsSvc<T: VpcVpc>(pub Arc<T>);
                    impl<T: VpcVpc> tonic::server::UnaryService<super::DescribeVpcsReq> for DescribeVpcsSvc<T> {
                        type Response = super::DescribeVpcsResp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DescribeVpcsReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).describe_vpcs(request).await };
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
                        let method = DescribeVpcsSvc(inner);
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
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: VpcVpc> Clone for VpcVpcServer<T> {
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
    impl<T: VpcVpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: VpcVpc> tonic::server::NamedService for VpcVpcServer<T> {
        const NAME: &'static str = "vpc_vpc.VpcVpc";
    }
}
