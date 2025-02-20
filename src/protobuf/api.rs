/// HTTP API定义
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRule {
    /// 和Google保持一致
    #[prost(string, tag = "1")]
    pub selector: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub body: ::prost::alloc::string::String,
    /// API描述 给API文档使用
    #[prost(string, tag = "8")]
    pub description: ::prost::alloc::string::String,
    /// 暂时没有用到
    #[prost(string, tag = "12")]
    pub response_body: ::prost::alloc::string::String,
    /// 和Google保持一致
    #[prost(message, repeated, tag = "11")]
    pub additional_bindings: ::prost::alloc::vec::Vec<HttpRule>,
    #[prost(oneof = "http_rule::Pattern", tags = "2, 3, 4, 5, 6")]
    pub pattern: ::core::option::Option<http_rule::Pattern>,
}
/// Nested message and enum types in `HttpRule`.
pub mod http_rule {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "PascalCase")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Pattern {
        #[prost(string, tag = "2")]
        Get(::prost::alloc::string::String),
        #[prost(string, tag = "3")]
        Put(::prost::alloc::string::String),
        #[prost(string, tag = "4")]
        Post(::prost::alloc::string::String),
        #[prost(string, tag = "5")]
        Delete(::prost::alloc::string::String),
        #[prost(string, tag = "6")]
        Patch(::prost::alloc::string::String),
    }
}
/// HTTP
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Http {
    #[prost(message, repeated, tag = "1")]
    pub rules: ::prost::alloc::vec::Vec<HttpRule>,
}
