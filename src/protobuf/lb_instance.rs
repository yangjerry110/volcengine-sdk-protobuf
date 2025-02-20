#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeLoadBalancersReq {
    #[prost(string, tag = "1")]
    pub exclusive_cluster_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub load_balancer_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub master_zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub address_ip_version: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub vpc_id: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub eni_address: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub eip_address: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub project_name: ::prost::alloc::string::String,
    #[prost(int64, tag = "11")]
    pub page_number: i64,
    #[prost(int64, tag = "12")]
    pub page_size: i64,
    #[prost(string, repeated, tag = "13")]
    pub load_balancer_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "14")]
    pub tag_filters: ::prost::alloc::vec::Vec<DescribeLoadBalancersTagFilterReq>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeLoadBalancersTagFilterReq {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeLoadBalancersResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<DescribeLoadBalancersResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeLoadBalancersResultResp {
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub total_count: i64,
    #[prost(int64, tag = "3")]
    pub page_number: i64,
    #[prost(int64, tag = "4")]
    pub page_size: i64,
    #[prost(message, repeated, tag = "5")]
    pub load_balancers: ::prost::alloc::vec::Vec<DescribeLoadBalancersResultLoadBalancerResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeLoadBalancersResultLoadBalancerResp {
    #[prost(string, tag = "1")]
    pub exclusive_cluster_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub load_balancer_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub load_balancer_name: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub address_ip_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "8")]
    pub load_balancer_billing_type: i64,
    #[prost(string, tag = "9")]
    pub load_balancer_spec: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub master_zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub slave_zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub vpc_id: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub subnet_id: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub eni_id: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub eni_address: ::prost::alloc::string::String,
    #[prost(int64, tag = "16")]
    pub eni_address_num: i64,
    #[prost(string, tag = "17")]
    pub eip_id: ::prost::alloc::string::String,
    #[prost(string, tag = "18")]
    pub eip_address: ::prost::alloc::string::String,
    #[prost(string, tag = "19")]
    pub eni_ipv6_address: ::prost::alloc::string::String,
    #[prost(string, tag = "20")]
    pub ipv6_eip_id: ::prost::alloc::string::String,
    #[prost(string, tag = "21")]
    pub modification_protection_status: ::prost::alloc::string::String,
    #[prost(string, tag = "22")]
    pub modification_protection_reason: ::prost::alloc::string::String,
    #[prost(bool, tag = "23")]
    pub service_managed: bool,
    #[prost(string, tag = "24")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag = "25")]
    pub business_status: ::prost::alloc::string::String,
    #[prost(string, tag = "26")]
    pub lock_reason: ::prost::alloc::string::String,
    #[prost(string, tag = "27")]
    pub project_name: ::prost::alloc::string::String,
    #[prost(string, tag = "28")]
    pub create_time: ::prost::alloc::string::String,
    #[prost(string, tag = "29")]
    pub update_time: ::prost::alloc::string::String,
    #[prost(string, tag = "30")]
    pub overdue_time: ::prost::alloc::string::String,
    #[prost(string, tag = "31")]
    pub deleted_time: ::prost::alloc::string::String,
    #[prost(string, tag = "32")]
    pub expired_time: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "33")]
    pub tags: ::prost::alloc::vec::Vec<DescribeLoadBalancersResultLoadBalancerTagResp>,
    #[prost(message, optional, tag = "34")]
    pub eni_addresses:
        ::core::option::Option<DescribeLoadBalancersResultLoadBalancerEniAddresseResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeLoadBalancersResultLoadBalancerTagResp {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeLoadBalancersResultLoadBalancerEniAddresseResp {
    #[prost(string, tag = "1")]
    pub eni_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub eip_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub eip_address: ::prost::alloc::string::String,
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
pub mod lb_instance_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct LbInstanceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LbInstanceClient<tonic::transport::Channel> {
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
    impl<T> LbInstanceClient<T>
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
        ) -> LbInstanceClient<InterceptedService<T, F>>
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
            LbInstanceClient::new(InterceptedService::new(inner, interceptor))
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
        /// DescribeLoadBalancers
        pub async fn describe_load_balancers(
            &mut self,
            request: impl tonic::IntoRequest<super::DescribeLoadBalancersReq>,
        ) -> std::result::Result<tonic::Response<super::DescribeLoadBalancersResp>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lb_instance.LbInstance/DescribeLoadBalancers",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "lb_instance.LbInstance",
                "DescribeLoadBalancers",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod lb_instance_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with LbInstanceServer.
    #[async_trait]
    pub trait LbInstance: Send + Sync + 'static {
        /// DescribeLoadBalancers
        async fn describe_load_balancers(
            &self,
            request: tonic::Request<super::DescribeLoadBalancersReq>,
        ) -> std::result::Result<tonic::Response<super::DescribeLoadBalancersResp>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct LbInstanceServer<T: LbInstance> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: LbInstance> LbInstanceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for LbInstanceServer<T>
    where
        T: LbInstance,
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
                "/lb_instance.LbInstance/DescribeLoadBalancers" => {
                    #[allow(non_camel_case_types)]
                    struct DescribeLoadBalancersSvc<T: LbInstance>(pub Arc<T>);
                    impl<T: LbInstance> tonic::server::UnaryService<super::DescribeLoadBalancersReq>
                        for DescribeLoadBalancersSvc<T>
                    {
                        type Response = super::DescribeLoadBalancersResp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DescribeLoadBalancersReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).describe_load_balancers(request).await };
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
                        let method = DescribeLoadBalancersSvc(inner);
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
    impl<T: LbInstance> Clone for LbInstanceServer<T> {
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
    impl<T: LbInstance> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: LbInstance> tonic::server::NamedService for LbInstanceServer<T> {
        const NAME: &'static str = "lb_instance.LbInstance";
    }
}
