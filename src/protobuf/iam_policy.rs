#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPoliciesReq {
    #[prost(string, tag = "1")]
    pub scope: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub with_service_role_policy: i64,
    #[prost(int64, tag = "3")]
    pub limit: i64,
    #[prost(int64, tag = "4")]
    pub offset: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPoliciesResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<ListPoliciesResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPoliciesResultResp {
    #[prost(int64, tag = "1")]
    pub limit: i64,
    #[prost(int64, tag = "2")]
    pub offset: i64,
    #[prost(int64, tag = "3")]
    pub total: i64,
    #[prost(message, repeated, tag = "4")]
    pub policy_metadata: ::prost::alloc::vec::Vec<ListPoliciesResultPolicyResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPoliciesResultPolicyResp {
    #[prost(string, tag = "1")]
    pub policy_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub policy_trn: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub policy_type: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub policy_document: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub category: ::prost::alloc::string::String,
    #[prost(int64, tag = "7")]
    pub attachment_count: i64,
    #[prost(int64, tag = "8")]
    pub is_service_role_policy: i64,
    #[prost(string, tag = "9")]
    pub create_date: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub update_date: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePolicyReq {
    #[prost(string, tag = "1")]
    pub policy_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePolicyResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAttachedUserPoliciesReq {
    #[prost(string, tag = "1")]
    pub user_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAttachedUserPoliciesResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<ListAttachedUserPoliciesResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAttachedUserPoliciesResultResp {
    #[prost(message, repeated, tag = "1")]
    pub attached_policy_metadata: ::prost::alloc::vec::Vec<
        ListAttachedUserPoliciesResultPolicyResp,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAttachedUserPoliciesResultPolicyResp {
    #[prost(string, tag = "1")]
    pub policy_trn: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub policy_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub policy_type: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub attach_date: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "6")]
    pub policy_scope: ::prost::alloc::vec::Vec<
        ListAttachedUserPoliciesResultPolicyScopeResp,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAttachedUserPoliciesResultPolicyScopeResp {
    #[prost(string, tag = "1")]
    pub policy_scope_type: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub project_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub project_display_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub attach_date: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetachUserPolicyReq {
    #[prost(string, tag = "1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub policy_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub policy_type: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetachUserPolicyResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttachUserPolicyReq {
    #[prost(string, tag = "1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub policy_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub policy_type: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttachUserPolicyResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePolicyReq {
    #[prost(string, tag = "1")]
    pub new_description: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_policy_document: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub new_policy_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub policy_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePolicyResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<UpdatePolicyResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePolicyResultResp {
    #[prost(message, optional, tag = "1")]
    pub policy: ::core::option::Option<UpdatePolicyResultPolicyResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePolicyResultPolicyResp {
    #[prost(string, optional, tag = "1")]
    pub policy_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub policy_trn: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub policy_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub policy_document: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub category: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "7")]
    pub attachment_count: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "8")]
    pub is_service_role_policy: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "9")]
    pub create_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub update_date: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPolicyReq {
    #[prost(string, tag = "1")]
    pub policy_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub policy_type: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPolicyResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<GetPolicyResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPolicyResultResp {
    #[prost(message, optional, tag = "1")]
    pub policy: ::core::option::Option<GetPolicyResultPolicyResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPolicyResultPolicyResp {
    #[prost(string, tag = "1")]
    pub policy_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub policy_trn: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub policy_type: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub policy_document: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub category: ::prost::alloc::string::String,
    #[prost(int64, tag = "7")]
    pub attachment_count: i64,
    #[prost(int64, tag = "8")]
    pub is_service_role_policy: i64,
    #[prost(string, tag = "9")]
    pub create_date: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub update_date: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePolicyReq {
    #[prost(string, tag = "1")]
    pub policy_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub policy_document: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePolicyResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<CreatePolicyResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePolicyResultResp {
    #[prost(message, optional, tag = "1")]
    pub policy: ::core::option::Option<CreatePolicyResultPolicyResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePolicyResultPolicyResp {
    #[prost(string, optional, tag = "1")]
    pub policy_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub policy_trn: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub policy_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub policy_document: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub category: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "7")]
    pub attachment_count: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "8")]
    pub is_service_role_policy: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "9")]
    pub site: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub create_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub update_date: ::core::option::Option<::prost::alloc::string::String>,
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
pub mod iam_policy_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct IamPolicyClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IamPolicyClient<tonic::transport::Channel> {
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
    impl<T> IamPolicyClient<T>
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
        ) -> IamPolicyClient<InterceptedService<T, F>>
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
            IamPolicyClient::new(InterceptedService::new(inner, interceptor))
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
        /// ListPolicies
        pub async fn list_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPoliciesReq>,
        ) -> std::result::Result<
            tonic::Response<super::ListPoliciesResp>,
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
                "/iam_policy.IamPolicy/ListPolicies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("iam_policy.IamPolicy", "ListPolicies"));
            self.inner.unary(req, path, codec).await
        }
        /// CreatePolicy
        pub async fn create_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePolicyReq>,
        ) -> std::result::Result<
            tonic::Response<super::CreatePolicyResp>,
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
                "/iam_policy.IamPolicy/CreatePolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("iam_policy.IamPolicy", "CreatePolicy"));
            self.inner.unary(req, path, codec).await
        }
        /// GetPolicy
        pub async fn get_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPolicyReq>,
        ) -> std::result::Result<tonic::Response<super::GetPolicyResp>, tonic::Status> {
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
                "/iam_policy.IamPolicy/GetPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("iam_policy.IamPolicy", "GetPolicy"));
            self.inner.unary(req, path, codec).await
        }
        /// UpdatePolicy
        pub async fn update_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePolicyReq>,
        ) -> std::result::Result<
            tonic::Response<super::UpdatePolicyResp>,
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
                "/iam_policy.IamPolicy/UpdatePolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("iam_policy.IamPolicy", "UpdatePolicy"));
            self.inner.unary(req, path, codec).await
        }
        /// AttachUserPolicy
        pub async fn attach_user_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::AttachUserPolicyReq>,
        ) -> std::result::Result<
            tonic::Response<super::AttachUserPolicyResp>,
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
                "/iam_policy.IamPolicy/AttachUserPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("iam_policy.IamPolicy", "AttachUserPolicy"));
            self.inner.unary(req, path, codec).await
        }
        /// DetachUserPolicy
        pub async fn detach_user_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DetachUserPolicyReq>,
        ) -> std::result::Result<
            tonic::Response<super::DetachUserPolicyResp>,
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
                "/iam_policy.IamPolicy/DetachUserPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("iam_policy.IamPolicy", "DetachUserPolicy"));
            self.inner.unary(req, path, codec).await
        }
        /// ListAttachedUserPolicies
        pub async fn list_attached_user_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAttachedUserPoliciesReq>,
        ) -> std::result::Result<
            tonic::Response<super::ListAttachedUserPoliciesResp>,
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
                "/iam_policy.IamPolicy/ListAttachedUserPolicies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("iam_policy.IamPolicy", "ListAttachedUserPolicies"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DeletePolicy
        pub async fn delete_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePolicyReq>,
        ) -> std::result::Result<
            tonic::Response<super::DeletePolicyResp>,
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
                "/iam_policy.IamPolicy/DeletePolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("iam_policy.IamPolicy", "DeletePolicy"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod iam_policy_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with IamPolicyServer.
    #[async_trait]
    pub trait IamPolicy: Send + Sync + 'static {
        /// ListPolicies
        async fn list_policies(
            &self,
            request: tonic::Request<super::ListPoliciesReq>,
        ) -> std::result::Result<
            tonic::Response<super::ListPoliciesResp>,
            tonic::Status,
        >;
        /// CreatePolicy
        async fn create_policy(
            &self,
            request: tonic::Request<super::CreatePolicyReq>,
        ) -> std::result::Result<
            tonic::Response<super::CreatePolicyResp>,
            tonic::Status,
        >;
        /// GetPolicy
        async fn get_policy(
            &self,
            request: tonic::Request<super::GetPolicyReq>,
        ) -> std::result::Result<tonic::Response<super::GetPolicyResp>, tonic::Status>;
        /// UpdatePolicy
        async fn update_policy(
            &self,
            request: tonic::Request<super::UpdatePolicyReq>,
        ) -> std::result::Result<
            tonic::Response<super::UpdatePolicyResp>,
            tonic::Status,
        >;
        /// AttachUserPolicy
        async fn attach_user_policy(
            &self,
            request: tonic::Request<super::AttachUserPolicyReq>,
        ) -> std::result::Result<
            tonic::Response<super::AttachUserPolicyResp>,
            tonic::Status,
        >;
        /// DetachUserPolicy
        async fn detach_user_policy(
            &self,
            request: tonic::Request<super::DetachUserPolicyReq>,
        ) -> std::result::Result<
            tonic::Response<super::DetachUserPolicyResp>,
            tonic::Status,
        >;
        /// ListAttachedUserPolicies
        async fn list_attached_user_policies(
            &self,
            request: tonic::Request<super::ListAttachedUserPoliciesReq>,
        ) -> std::result::Result<
            tonic::Response<super::ListAttachedUserPoliciesResp>,
            tonic::Status,
        >;
        /// DeletePolicy
        async fn delete_policy(
            &self,
            request: tonic::Request<super::DeletePolicyReq>,
        ) -> std::result::Result<
            tonic::Response<super::DeletePolicyResp>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct IamPolicyServer<T: IamPolicy> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: IamPolicy> IamPolicyServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for IamPolicyServer<T>
    where
        T: IamPolicy,
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
                "/iam_policy.IamPolicy/ListPolicies" => {
                    #[allow(non_camel_case_types)]
                    struct ListPoliciesSvc<T: IamPolicy>(pub Arc<T>);
                    impl<
                        T: IamPolicy,
                    > tonic::server::UnaryService<super::ListPoliciesReq>
                    for ListPoliciesSvc<T> {
                        type Response = super::ListPoliciesResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPoliciesReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_policies(request).await
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
                        let method = ListPoliciesSvc(inner);
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
                "/iam_policy.IamPolicy/CreatePolicy" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePolicySvc<T: IamPolicy>(pub Arc<T>);
                    impl<
                        T: IamPolicy,
                    > tonic::server::UnaryService<super::CreatePolicyReq>
                    for CreatePolicySvc<T> {
                        type Response = super::CreatePolicyResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreatePolicyReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_policy(request).await
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
                        let method = CreatePolicySvc(inner);
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
                "/iam_policy.IamPolicy/GetPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetPolicySvc<T: IamPolicy>(pub Arc<T>);
                    impl<T: IamPolicy> tonic::server::UnaryService<super::GetPolicyReq>
                    for GetPolicySvc<T> {
                        type Response = super::GetPolicyResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPolicyReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_policy(request).await };
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
                        let method = GetPolicySvc(inner);
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
                "/iam_policy.IamPolicy/UpdatePolicy" => {
                    #[allow(non_camel_case_types)]
                    struct UpdatePolicySvc<T: IamPolicy>(pub Arc<T>);
                    impl<
                        T: IamPolicy,
                    > tonic::server::UnaryService<super::UpdatePolicyReq>
                    for UpdatePolicySvc<T> {
                        type Response = super::UpdatePolicyResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdatePolicyReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_policy(request).await
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
                        let method = UpdatePolicySvc(inner);
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
                "/iam_policy.IamPolicy/AttachUserPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct AttachUserPolicySvc<T: IamPolicy>(pub Arc<T>);
                    impl<
                        T: IamPolicy,
                    > tonic::server::UnaryService<super::AttachUserPolicyReq>
                    for AttachUserPolicySvc<T> {
                        type Response = super::AttachUserPolicyResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AttachUserPolicyReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).attach_user_policy(request).await
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
                        let method = AttachUserPolicySvc(inner);
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
                "/iam_policy.IamPolicy/DetachUserPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct DetachUserPolicySvc<T: IamPolicy>(pub Arc<T>);
                    impl<
                        T: IamPolicy,
                    > tonic::server::UnaryService<super::DetachUserPolicyReq>
                    for DetachUserPolicySvc<T> {
                        type Response = super::DetachUserPolicyResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DetachUserPolicyReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).detach_user_policy(request).await
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
                        let method = DetachUserPolicySvc(inner);
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
                "/iam_policy.IamPolicy/ListAttachedUserPolicies" => {
                    #[allow(non_camel_case_types)]
                    struct ListAttachedUserPoliciesSvc<T: IamPolicy>(pub Arc<T>);
                    impl<
                        T: IamPolicy,
                    > tonic::server::UnaryService<super::ListAttachedUserPoliciesReq>
                    for ListAttachedUserPoliciesSvc<T> {
                        type Response = super::ListAttachedUserPoliciesResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAttachedUserPoliciesReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_attached_user_policies(request).await
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
                        let method = ListAttachedUserPoliciesSvc(inner);
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
                "/iam_policy.IamPolicy/DeletePolicy" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePolicySvc<T: IamPolicy>(pub Arc<T>);
                    impl<
                        T: IamPolicy,
                    > tonic::server::UnaryService<super::DeletePolicyReq>
                    for DeletePolicySvc<T> {
                        type Response = super::DeletePolicyResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeletePolicyReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_policy(request).await
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
                        let method = DeletePolicySvc(inner);
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
    impl<T: IamPolicy> Clone for IamPolicyServer<T> {
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
    impl<T: IamPolicy> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: IamPolicy> tonic::server::NamedService for IamPolicyServer<T> {
        const NAME: &'static str = "iam_policy.IamPolicy";
    }
}
