#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUserReq {
    #[prost(string, optional, tag = "1")]
    pub user_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUserResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSecurityConfigReq {
    #[prost(string, optional, tag = "1")]
    pub user_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSecurityConfigResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSecurityConfigResultResp {
    #[prost(string, optional, tag = "1")]
    pub safe_auth_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "2")]
    pub safe_auth_exempt_duration: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "3")]
    pub user_id: ::core::option::Option<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSecurityConfigReq {
    #[prost(string, optional, tag = "1")]
    pub user_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub safe_auth_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub safe_auth_exempt_duration: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSecurityConfigResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLoginProfileReq {
    #[prost(string, optional, tag = "1")]
    pub user_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLoginProfileResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLoginProfileReq {
    #[prost(string, optional, tag = "1")]
    pub user_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub password: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub login_allowed: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub password_reset_required: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "5")]
    pub safe_auth_flag: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "6")]
    pub safe_auth_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "7")]
    pub safe_auth_exempt_required: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "8")]
    pub safe_auth_exempt_unit: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "9")]
    pub safe_auth_exempt_duration: ::core::option::Option<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLoginProfileResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<UpdateLoginProfileResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLoginProfileResultResp {
    #[prost(message, optional, tag = "1")]
    pub login_profile: ::core::option::Option<UpdateLoginProfilResultLoginProfileResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLoginProfilResultLoginProfileResp {
    #[prost(int64, optional, tag = "1")]
    pub user_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub user_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub login_allowed: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub password_reset_required: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "5")]
    pub password_expire_at: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "6")]
    pub last_reset_password_time: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "7")]
    pub last_login_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub last_login_ip: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "9")]
    pub login_locked: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "10")]
    pub safe_auth_flag: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "11")]
    pub safe_auth_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "12")]
    pub safe_auth_exempt_required: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "13")]
    pub safe_auth_exempt_unit: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "14")]
    pub safe_auth_exempt_duration: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "15")]
    pub create_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "16")]
    pub update_date: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLoginProfileReq {
    #[prost(string, optional, tag = "1")]
    pub user_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLoginProfileResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<GetLoginProfileResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLoginProfileResultResp {
    #[prost(message, optional, tag = "1")]
    pub login_profile: ::core::option::Option<GetLoginProfileResultLoginProfileResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLoginProfileResultLoginProfileResp {
    #[prost(int64, optional, tag = "1")]
    pub user_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub user_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub login_allowed: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub password_reset_required: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "5")]
    pub password_expire_at: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "6")]
    pub last_reset_password_time: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "7")]
    pub last_login_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub last_login_ip: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "9")]
    pub login_locked: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "10")]
    pub safe_auth_flag: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "11")]
    pub safe_auth_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "12")]
    pub safe_auth_exempt_required: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "13")]
    pub safe_auth_exempt_unit: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "14")]
    pub safe_auth_exempt_duration: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "15")]
    pub create_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "16")]
    pub update_date: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLoginProfileReq {
    #[prost(string, optional, tag = "1")]
    pub user_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub login_allowed: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub password_reset_required: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "4")]
    pub password: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "5")]
    pub safe_auth_flag: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "6")]
    pub safe_auth_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "7")]
    pub safe_auth_exempt_duration: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "8")]
    pub safe_auth_exempt_required: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "9")]
    pub safe_auth_exempt_unit: ::core::option::Option<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLoginProfileResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<CreateLoginProfileResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLoginProfileResultResp {
    #[prost(message, optional, tag = "1")]
    pub login_profile: ::core::option::Option<CreateLoginProfileResultLoginProfileResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLoginProfileResultLoginProfileResp {
    #[prost(int64, optional, tag = "1")]
    pub user_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub user_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub login_allowed: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub password_reset_required: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "5")]
    pub password_expire_at: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "6")]
    pub last_reset_password_time: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "7")]
    pub last_login_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub last_login_ip: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "9")]
    pub login_locked: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "10")]
    pub safe_auth_flag: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "11")]
    pub safe_auth_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "12")]
    pub safe_auth_exempt_required: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "13")]
    pub safe_auth_exempt_unit: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "14")]
    pub safe_auth_exempt_duration: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "15")]
    pub create_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "16")]
    pub update_date: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserReq {
    #[prost(string, optional, tag = "1")]
    pub user_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub new_user_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub new_display_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub new_description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub new_mobile_phone: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub new_email: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<GetUserResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserResultResp {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<UpdateUserResultUserResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserResultUserResp {
    #[prost(string, optional, tag = "1")]
    pub create_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub update_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "4")]
    pub account_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "5")]
    pub user_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub display_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub email: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "9")]
    pub email_is_verify: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "10")]
    pub mobile_phone: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "11")]
    pub mobile_phone_is_verify: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "12")]
    pub trn: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserReq {
    #[prost(string, optional, tag = "1")]
    pub user_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub display_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub email: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub mobile_phone: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "6")]
    pub tags: ::prost::alloc::vec::Vec<CreateUserTagReq>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserTagReq {
    #[prost(string, optional, tag = "1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserReq {
    #[prost(string, optional, tag = "1")]
    pub user_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub access_key_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<CreateUserResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserResultResp {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<CreateUserResultUserResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserResultUserResp {
    #[prost(string, optional, tag = "1")]
    pub create_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub update_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "4")]
    pub account_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "5")]
    pub user_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub display_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub email: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "9")]
    pub email_is_verify: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "10")]
    pub mobile_phone: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "11")]
    pub mobile_phone_is_verify: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "12")]
    pub trn: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<GetUserResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserResultResp {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<GetUserResultUserResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserResultUserResp {
    #[prost(string, optional, tag = "1")]
    pub create_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub update_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "4")]
    pub account_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "5")]
    pub user_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub display_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub email: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "9")]
    pub email_is_verify: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "10")]
    pub mobile_phone: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "11")]
    pub mobile_phone_is_verify: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "12")]
    pub trn: ::core::option::Option<::prost::alloc::string::String>,
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
pub mod iam_user_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct IamUserClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IamUserClient<tonic::transport::Channel> {
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
    impl<T> IamUserClient<T>
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
        ) -> IamUserClient<InterceptedService<T, F>>
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
            IamUserClient::new(InterceptedService::new(inner, interceptor))
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
        /// get_user
        pub async fn get_user(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserReq>,
        ) -> std::result::Result<tonic::Response<super::GetUserResp>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/iam_user.IamUser/GetUser");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("iam_user.IamUser", "GetUser"));
            self.inner.unary(req, path, codec).await
        }
        /// create_user
        pub async fn create_user(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUserReq>,
        ) -> std::result::Result<tonic::Response<super::CreateUserResp>, tonic::Status> {
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
                "/iam_user.IamUser/CreateUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("iam_user.IamUser", "CreateUser"));
            self.inner.unary(req, path, codec).await
        }
        /// update user
        pub async fn update_user(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserReq>,
        ) -> std::result::Result<tonic::Response<super::UpdateUserResp>, tonic::Status> {
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
                "/iam_user.IamUser/UpdateUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("iam_user.IamUser", "UpdateUser"));
            self.inner.unary(req, path, codec).await
        }
        /// delete user
        pub async fn delete_user(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteUserReq>,
        ) -> std::result::Result<tonic::Response<super::DeleteUserResp>, tonic::Status> {
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
                "/iam_user.IamUser/DeleteUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("iam_user.IamUser", "DeleteUser"));
            self.inner.unary(req, path, codec).await
        }
        /// CreateLoginProfile
        pub async fn create_login_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateLoginProfileReq>,
        ) -> std::result::Result<
            tonic::Response<super::CreateLoginProfileResp>,
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
                "/iam_user.IamUser/CreateLoginProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("iam_user.IamUser", "CreateLoginProfile"));
            self.inner.unary(req, path, codec).await
        }
        /// GetLoginProfile
        pub async fn get_login_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLoginProfileReq>,
        ) -> std::result::Result<
            tonic::Response<super::GetLoginProfileResp>,
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
                "/iam_user.IamUser/GetLoginProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("iam_user.IamUser", "GetLoginProfile"));
            self.inner.unary(req, path, codec).await
        }
        /// UpdateLoginProfile
        pub async fn update_login_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateLoginProfileReq>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateLoginProfileResp>,
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
                "/iam_user.IamUser/UpdateLoginProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("iam_user.IamUser", "UpdateLoginProfile"));
            self.inner.unary(req, path, codec).await
        }
        /// DeleteLoginProfile
        pub async fn delete_login_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteLoginProfileReq>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteLoginProfileResp>,
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
                "/iam_user.IamUser/DeleteLoginProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("iam_user.IamUser", "DeleteLoginProfile"));
            self.inner.unary(req, path, codec).await
        }
        /// SetSecurityConfig
        pub async fn set_security_config(
            &mut self,
            request: impl tonic::IntoRequest<super::SetSecurityConfigReq>,
        ) -> std::result::Result<
            tonic::Response<super::SetSecurityConfigResp>,
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
                "/iam_user.IamUser/SetSecurityConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("iam_user.IamUser", "SetSecurityConfig"));
            self.inner.unary(req, path, codec).await
        }
        /// GetSecurityConfig
        pub async fn get_security_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSecurityConfigReq>,
        ) -> std::result::Result<
            tonic::Response<super::GetSecurityConfigResp>,
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
                "/iam_user.IamUser/GetSecurityConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("iam_user.IamUser", "GetSecurityConfig"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod iam_user_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with IamUserServer.
    #[async_trait]
    pub trait IamUser: Send + Sync + 'static {
        /// get_user
        async fn get_user(
            &self,
            request: tonic::Request<super::GetUserReq>,
        ) -> std::result::Result<tonic::Response<super::GetUserResp>, tonic::Status>;
        /// create_user
        async fn create_user(
            &self,
            request: tonic::Request<super::CreateUserReq>,
        ) -> std::result::Result<tonic::Response<super::CreateUserResp>, tonic::Status>;
        /// update user
        async fn update_user(
            &self,
            request: tonic::Request<super::UpdateUserReq>,
        ) -> std::result::Result<tonic::Response<super::UpdateUserResp>, tonic::Status>;
        /// delete user
        async fn delete_user(
            &self,
            request: tonic::Request<super::DeleteUserReq>,
        ) -> std::result::Result<tonic::Response<super::DeleteUserResp>, tonic::Status>;
        /// CreateLoginProfile
        async fn create_login_profile(
            &self,
            request: tonic::Request<super::CreateLoginProfileReq>,
        ) -> std::result::Result<
            tonic::Response<super::CreateLoginProfileResp>,
            tonic::Status,
        >;
        /// GetLoginProfile
        async fn get_login_profile(
            &self,
            request: tonic::Request<super::GetLoginProfileReq>,
        ) -> std::result::Result<
            tonic::Response<super::GetLoginProfileResp>,
            tonic::Status,
        >;
        /// UpdateLoginProfile
        async fn update_login_profile(
            &self,
            request: tonic::Request<super::UpdateLoginProfileReq>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateLoginProfileResp>,
            tonic::Status,
        >;
        /// DeleteLoginProfile
        async fn delete_login_profile(
            &self,
            request: tonic::Request<super::DeleteLoginProfileReq>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteLoginProfileResp>,
            tonic::Status,
        >;
        /// SetSecurityConfig
        async fn set_security_config(
            &self,
            request: tonic::Request<super::SetSecurityConfigReq>,
        ) -> std::result::Result<
            tonic::Response<super::SetSecurityConfigResp>,
            tonic::Status,
        >;
        /// GetSecurityConfig
        async fn get_security_config(
            &self,
            request: tonic::Request<super::GetSecurityConfigReq>,
        ) -> std::result::Result<
            tonic::Response<super::GetSecurityConfigResp>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct IamUserServer<T: IamUser> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: IamUser> IamUserServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for IamUserServer<T>
    where
        T: IamUser,
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
                "/iam_user.IamUser/GetUser" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserSvc<T: IamUser>(pub Arc<T>);
                    impl<T: IamUser> tonic::server::UnaryService<super::GetUserReq>
                    for GetUserSvc<T> {
                        type Response = super::GetUserResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_user(request).await };
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
                        let method = GetUserSvc(inner);
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
                "/iam_user.IamUser/CreateUser" => {
                    #[allow(non_camel_case_types)]
                    struct CreateUserSvc<T: IamUser>(pub Arc<T>);
                    impl<T: IamUser> tonic::server::UnaryService<super::CreateUserReq>
                    for CreateUserSvc<T> {
                        type Response = super::CreateUserResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateUserReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).create_user(request).await };
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
                        let method = CreateUserSvc(inner);
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
                "/iam_user.IamUser/UpdateUser" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateUserSvc<T: IamUser>(pub Arc<T>);
                    impl<T: IamUser> tonic::server::UnaryService<super::UpdateUserReq>
                    for UpdateUserSvc<T> {
                        type Response = super::UpdateUserResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateUserReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).update_user(request).await };
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
                        let method = UpdateUserSvc(inner);
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
                "/iam_user.IamUser/DeleteUser" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteUserSvc<T: IamUser>(pub Arc<T>);
                    impl<T: IamUser> tonic::server::UnaryService<super::DeleteUserReq>
                    for DeleteUserSvc<T> {
                        type Response = super::DeleteUserResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteUserReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delete_user(request).await };
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
                        let method = DeleteUserSvc(inner);
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
                "/iam_user.IamUser/CreateLoginProfile" => {
                    #[allow(non_camel_case_types)]
                    struct CreateLoginProfileSvc<T: IamUser>(pub Arc<T>);
                    impl<
                        T: IamUser,
                    > tonic::server::UnaryService<super::CreateLoginProfileReq>
                    for CreateLoginProfileSvc<T> {
                        type Response = super::CreateLoginProfileResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateLoginProfileReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_login_profile(request).await
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
                        let method = CreateLoginProfileSvc(inner);
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
                "/iam_user.IamUser/GetLoginProfile" => {
                    #[allow(non_camel_case_types)]
                    struct GetLoginProfileSvc<T: IamUser>(pub Arc<T>);
                    impl<
                        T: IamUser,
                    > tonic::server::UnaryService<super::GetLoginProfileReq>
                    for GetLoginProfileSvc<T> {
                        type Response = super::GetLoginProfileResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetLoginProfileReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_login_profile(request).await
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
                        let method = GetLoginProfileSvc(inner);
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
                "/iam_user.IamUser/UpdateLoginProfile" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateLoginProfileSvc<T: IamUser>(pub Arc<T>);
                    impl<
                        T: IamUser,
                    > tonic::server::UnaryService<super::UpdateLoginProfileReq>
                    for UpdateLoginProfileSvc<T> {
                        type Response = super::UpdateLoginProfileResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateLoginProfileReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_login_profile(request).await
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
                        let method = UpdateLoginProfileSvc(inner);
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
                "/iam_user.IamUser/DeleteLoginProfile" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteLoginProfileSvc<T: IamUser>(pub Arc<T>);
                    impl<
                        T: IamUser,
                    > tonic::server::UnaryService<super::DeleteLoginProfileReq>
                    for DeleteLoginProfileSvc<T> {
                        type Response = super::DeleteLoginProfileResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteLoginProfileReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_login_profile(request).await
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
                        let method = DeleteLoginProfileSvc(inner);
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
                "/iam_user.IamUser/SetSecurityConfig" => {
                    #[allow(non_camel_case_types)]
                    struct SetSecurityConfigSvc<T: IamUser>(pub Arc<T>);
                    impl<
                        T: IamUser,
                    > tonic::server::UnaryService<super::SetSecurityConfigReq>
                    for SetSecurityConfigSvc<T> {
                        type Response = super::SetSecurityConfigResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetSecurityConfigReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_security_config(request).await
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
                        let method = SetSecurityConfigSvc(inner);
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
                "/iam_user.IamUser/GetSecurityConfig" => {
                    #[allow(non_camel_case_types)]
                    struct GetSecurityConfigSvc<T: IamUser>(pub Arc<T>);
                    impl<
                        T: IamUser,
                    > tonic::server::UnaryService<super::GetSecurityConfigReq>
                    for GetSecurityConfigSvc<T> {
                        type Response = super::GetSecurityConfigResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSecurityConfigReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_security_config(request).await
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
                        let method = GetSecurityConfigSvc(inner);
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
    impl<T: IamUser> Clone for IamUserServer<T> {
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
    impl<T: IamUser> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: IamUser> tonic::server::NamedService for IamUserServer<T> {
        const NAME: &'static str = "iam_user.IamUser";
    }
}
