#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisDescribeDbInstancesReq {
    #[prost(int64, optional, tag = "1")]
    pub page_number: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub page_size: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "3")]
    pub region_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub zone_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub instance_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub instance_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "7")]
    pub sharded_cluster: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "8")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub engine_version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub vpc_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub charge_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub project_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "13")]
    pub service_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "14")]
    pub data_layout: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "15")]
    pub tag_filters: ::prost::alloc::vec::Vec<RedisDescribeDbInstancesTagReq>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisDescribeDbInstancesTagReq {
    #[prost(string, optional, tag = "1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisDescribeDbInstancesResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<RedisDescribeDbInstancesResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisDescribeDbInstancesResultResp {
    #[prost(int64, optional, tag = "1")]
    pub total_instances_num: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "2")]
    #[serde(default)]
    pub instances: ::prost::alloc::vec::Vec<RedisDescribeDbInstancesResultInstanceResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisDescribeDbInstancesResultInstanceResp {
    #[prost(string, optional, tag = "1")]
    pub charge_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub deletion_protection: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub engine_version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub expired_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub instance_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub instance_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub project_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub region_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    #[serde(rename = "MultiAZ")]
    pub multi_az: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub vpc_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "13")]
    pub node_number: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "14")]
    pub shard_capacity: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "15")]
    pub shard_number: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "16")]
    pub sharded_cluster: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "17")]
    pub instance_class: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "18")]
    pub service_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "19")]
    pub data_layout: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "20")]
    pub private_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "21")]
    #[serde(rename = "VIP")]
    pub vip: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "22")]
    #[serde(rename = "VIPv6")]
    pub vi_pv6: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "23")]
    pub capacity: ::core::option::Option<
        RedisDescribeDbInstancesResultInstanceCapacityResp,
    >,
    #[prost(string, repeated, tag = "24")]
    pub zone_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "25")]
    pub tags: ::prost::alloc::vec::Vec<RedisDescribeDbInstanceDetailResultTagResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisDescribeDbInstancesResultInstanceTagResp {
    #[prost(string, optional, tag = "1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisDescribeDbInstancesResultInstanceCapacityResp {
    #[prost(int64, optional, tag = "1")]
    pub used: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub total: ::core::option::Option<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisDescribeDbInstanceDetailReq {
    #[prost(string, tag = "1")]
    pub instance_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisDescribeDbInstanceDetailResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<RedisDescribeDbInstanceDetailResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisDescribeDbInstanceDetailResultResp {
    #[prost(string, tag = "1")]
    pub region_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub instance_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub instance_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub engine_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub sharded_cluster: i64,
    #[prost(int64, tag = "7")]
    pub node_number: i64,
    #[prost(int64, tag = "8")]
    pub shard_number: i64,
    #[prost(int64, tag = "9")]
    pub shard_capacity_v2: i64,
    #[prost(string, tag = "10")]
    pub vpc_id: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub subnet_id: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub create_time: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub maintenance_time: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub expired_time: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub project_name: ::prost::alloc::string::String,
    #[prost(string, tag = "16")]
    pub deletion_protection: ::prost::alloc::string::String,
    #[prost(string, tag = "17")]
    pub vpc_auth_mode: ::prost::alloc::string::String,
    #[prost(string, tag = "18")]
    pub instance_class: ::prost::alloc::string::String,
    #[prost(string, tag = "19")]
    pub multi_az: ::prost::alloc::string::String,
    #[prost(string, tag = "20")]
    pub charge_type: ::prost::alloc::string::String,
    #[prost(bool, tag = "21")]
    pub auto_renew: bool,
    #[prost(int64, tag = "22")]
    pub max_connections: i64,
    #[prost(string, tag = "23")]
    pub data_layout: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "24")]
    pub capacity: ::core::option::Option<
        RedisDescribeDbInstanceDetailResultCapacityResp,
    >,
    #[prost(string, repeated, tag = "25")]
    pub zone_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "26")]
    pub visit_addrs: ::prost::alloc::vec::Vec<
        RedisDescribeDbInstanceDetailResultVisitAddrResp,
    >,
    #[prost(message, repeated, tag = "27")]
    pub tags: ::prost::alloc::vec::Vec<RedisDescribeDbInstanceDetailResultTagResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisDescribeDbInstanceDetailResultTagResp {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisDescribeDbInstanceDetailResultVisitAddrResp {
    #[prost(string, tag = "1")]
    pub port: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub eip_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub addr_type: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub vip: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub vi_pv6: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisDescribeDbInstanceDetailResultCapacityResp {
    #[prost(int64, tag = "1")]
    pub used: i64,
    #[prost(int64, tag = "2")]
    pub total: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisCreateDbInstanceReq {
    #[prost(string, tag = "1")]
    pub region_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub engine_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub sharded_cluster: i64,
    #[prost(int64, tag = "4")]
    pub shard_number: i64,
    #[prost(int64, tag = "5")]
    pub node_number: i64,
    #[prost(int64, tag = "6")]
    pub shard_capacity: i64,
    #[prost(string, tag = "7")]
    pub multi_az: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub vpc_id: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub subnet_id: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub instance_name: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub password: ::prost::alloc::string::String,
    #[prost(int64, tag = "12")]
    pub port: i64,
    #[prost(string, tag = "13")]
    pub project_name: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub charge_type: ::prost::alloc::string::String,
    #[prost(int64, tag = "15")]
    pub purchase_months: i64,
    #[prost(bool, tag = "16")]
    pub auto_renew: bool,
    #[prost(string, tag = "17")]
    pub deletion_protection: ::prost::alloc::string::String,
    #[prost(string, tag = "18")]
    pub client_token: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "19")]
    pub allow_list_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "20")]
    pub configure_nodes: ::prost::alloc::vec::Vec<RedisCreateDbInstanceConfigureNodeReq>,
    #[prost(message, repeated, tag = "21")]
    pub tags: ::prost::alloc::vec::Vec<RedisCreateDbInstanceTagReq>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisCreateDbInstanceTagReq {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisCreateDbInstanceConfigureNodeReq {
    #[prost(string, tag = "1")]
    pub az: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisCreateDbInstanceResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<RedisCreateDbInstanceResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisCreateDbInstanceResultResp {
    #[prost(string, tag = "1")]
    pub order_no: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisIncreaseDbInstanceNodeNumberReq {
    #[prost(string, optional, tag = "1")]
    pub instance_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub apply_immediately: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "3")]
    pub nodes_number_to_increase: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "4")]
    pub create_backup: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "5")]
    pub backup_point_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub client_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "7")]
    pub configure_new_nodes: ::prost::alloc::vec::Vec<
        RedisIncreaseDbInstanceNodeNumberConfigureNewNodeReq,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisIncreaseDbInstanceNodeNumberConfigureNewNodeReq {
    #[prost(string, optional, tag = "1")]
    pub az: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisIncreaseDbInstanceNodeNumberResultResp {
    #[prost(string, optional, tag = "1")]
    pub order_no: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisIncreaseDbInstanceNodeNumberResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<RedisIncreaseDbInstanceNodeNumberResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisDecreaseDbInstanceNodeNumberReq {
    #[prost(string, optional, tag = "1")]
    pub instance_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub apply_immediately: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "3")]
    pub nodes_number_to_increase: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "4")]
    pub create_backup: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "5")]
    pub backup_point_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub client_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "7")]
    pub configure_new_nodes: ::prost::alloc::vec::Vec<
        RedisIncreaseDbInstanceNodeNumberConfigureNewNodeReq,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisDecreaseDbInstanceNodeNumberResultResp {
    #[prost(string, optional, tag = "1")]
    pub order_no: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisDecreaseDbInstanceNodeNumberResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<RedisDecreaseDbInstanceNodeNumberResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisModifyDbInstanceShardCapacityReq {
    #[prost(string, optional, tag = "1")]
    pub instance_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub apply_immediately: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "3")]
    pub shard_capacity: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "4")]
    pub create_backup: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "5")]
    pub backup_point_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub client_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisModifyDbInstanceShardCapacityResultResp {
    #[prost(string, optional, tag = "1")]
    pub order_no: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisModifyDbInstanceShardCapacityResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<RedisModifyDbInstanceShardCapacityResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisModifyDbInstanceShardNumberReq {
    #[prost(string, optional, tag = "1")]
    pub instance_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub apply_immediately: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "3")]
    pub shard_number: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "4")]
    pub create_backup: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "5")]
    pub backup_point_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub client_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisModifyDbInstanceShardNumberResultResp {
    #[prost(string, optional, tag = "1")]
    pub order_no: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisModifyDbInstanceShardNumberResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<RedisModifyDbInstanceShardNumberResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisEnableShardedClusterReq {
    #[prost(string, optional, tag = "1")]
    pub instance_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub apply_immediately: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "3")]
    pub shard_number: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "4")]
    pub sharded_cluster: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "5")]
    pub shard_capacity: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "6")]
    pub create_backup: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "7")]
    pub backup_point_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub client_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisEnableShardedClusterResultResp {
    #[prost(string, optional, tag = "1")]
    pub order_no: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisEnableShardedClusterResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<RedisEnableShardedClusterResultResp>,
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
pub mod redis_instance_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct RedisInstanceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RedisInstanceClient<tonic::transport::Channel> {
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
    impl<T> RedisInstanceClient<T>
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
        ) -> RedisInstanceClient<InterceptedService<T, F>>
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
            RedisInstanceClient::new(InterceptedService::new(inner, interceptor))
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
        /// DescribeDBInstances
        pub async fn redis_describe_db_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::RedisDescribeDbInstancesReq>,
        ) -> std::result::Result<
            tonic::Response<super::RedisDescribeDbInstancesResp>,
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
                "/redis_instance.RedisInstance/RedisDescribeDBInstances",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "redis_instance.RedisInstance",
                        "RedisDescribeDBInstances",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// CreateDBInstance
        pub async fn redis_create_db_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::RedisCreateDbInstanceReq>,
        ) -> std::result::Result<
            tonic::Response<super::RedisCreateDbInstanceResp>,
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
                "/redis_instance.RedisInstance/RedisCreateDBInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "redis_instance.RedisInstance",
                        "RedisCreateDBInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DescribeDBInstanceDetail
        pub async fn redis_describe_db_instance_detail(
            &mut self,
            request: impl tonic::IntoRequest<super::RedisDescribeDbInstanceDetailReq>,
        ) -> std::result::Result<
            tonic::Response<super::RedisDescribeDbInstanceDetailResp>,
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
                "/redis_instance.RedisInstance/RedisDescribeDBInstanceDetail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "redis_instance.RedisInstance",
                        "RedisDescribeDBInstanceDetail",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// IncreaseDBInstanceNodeNumber
        pub async fn redis_increase_db_instance_node_number(
            &mut self,
            request: impl tonic::IntoRequest<super::RedisIncreaseDbInstanceNodeNumberReq>,
        ) -> std::result::Result<
            tonic::Response<super::RedisIncreaseDbInstanceNodeNumberResp>,
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
                "/redis_instance.RedisInstance/RedisIncreaseDBInstanceNodeNumber",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "redis_instance.RedisInstance",
                        "RedisIncreaseDBInstanceNodeNumber",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DecreaseDBInstanceNodeNumber
        pub async fn redis_decrease_db_instance_node_number(
            &mut self,
            request: impl tonic::IntoRequest<super::RedisDecreaseDbInstanceNodeNumberReq>,
        ) -> std::result::Result<
            tonic::Response<super::RedisDecreaseDbInstanceNodeNumberResp>,
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
                "/redis_instance.RedisInstance/RedisDecreaseDBInstanceNodeNumber",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "redis_instance.RedisInstance",
                        "RedisDecreaseDBInstanceNodeNumber",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ModifyDBInstanceShardCapacity
        pub async fn redis_modify_db_instance_shard_capacity(
            &mut self,
            request: impl tonic::IntoRequest<
                super::RedisModifyDbInstanceShardCapacityReq,
            >,
        ) -> std::result::Result<
            tonic::Response<super::RedisModifyDbInstanceShardCapacityResp>,
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
                "/redis_instance.RedisInstance/RedisModifyDBInstanceShardCapacity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "redis_instance.RedisInstance",
                        "RedisModifyDBInstanceShardCapacity",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ModifyDBInstanceShardNumber
        pub async fn redis_modify_db_instance_shard_number(
            &mut self,
            request: impl tonic::IntoRequest<super::RedisModifyDbInstanceShardNumberReq>,
        ) -> std::result::Result<
            tonic::Response<super::RedisModifyDbInstanceShardNumberResp>,
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
                "/redis_instance.RedisInstance/RedisModifyDBInstanceShardNumber",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "redis_instance.RedisInstance",
                        "RedisModifyDBInstanceShardNumber",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// EnableShardedCluster
        pub async fn redis_enable_sharded_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::RedisEnableShardedClusterReq>,
        ) -> std::result::Result<
            tonic::Response<super::RedisEnableShardedClusterResp>,
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
                "/redis_instance.RedisInstance/RedisEnableShardedCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "redis_instance.RedisInstance",
                        "RedisEnableShardedCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod redis_instance_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RedisInstanceServer.
    #[async_trait]
    pub trait RedisInstance: Send + Sync + 'static {
        /// DescribeDBInstances
        async fn redis_describe_db_instances(
            &self,
            request: tonic::Request<super::RedisDescribeDbInstancesReq>,
        ) -> std::result::Result<
            tonic::Response<super::RedisDescribeDbInstancesResp>,
            tonic::Status,
        >;
        /// CreateDBInstance
        async fn redis_create_db_instance(
            &self,
            request: tonic::Request<super::RedisCreateDbInstanceReq>,
        ) -> std::result::Result<
            tonic::Response<super::RedisCreateDbInstanceResp>,
            tonic::Status,
        >;
        /// DescribeDBInstanceDetail
        async fn redis_describe_db_instance_detail(
            &self,
            request: tonic::Request<super::RedisDescribeDbInstanceDetailReq>,
        ) -> std::result::Result<
            tonic::Response<super::RedisDescribeDbInstanceDetailResp>,
            tonic::Status,
        >;
        /// IncreaseDBInstanceNodeNumber
        async fn redis_increase_db_instance_node_number(
            &self,
            request: tonic::Request<super::RedisIncreaseDbInstanceNodeNumberReq>,
        ) -> std::result::Result<
            tonic::Response<super::RedisIncreaseDbInstanceNodeNumberResp>,
            tonic::Status,
        >;
        /// DecreaseDBInstanceNodeNumber
        async fn redis_decrease_db_instance_node_number(
            &self,
            request: tonic::Request<super::RedisDecreaseDbInstanceNodeNumberReq>,
        ) -> std::result::Result<
            tonic::Response<super::RedisDecreaseDbInstanceNodeNumberResp>,
            tonic::Status,
        >;
        /// ModifyDBInstanceShardCapacity
        async fn redis_modify_db_instance_shard_capacity(
            &self,
            request: tonic::Request<super::RedisModifyDbInstanceShardCapacityReq>,
        ) -> std::result::Result<
            tonic::Response<super::RedisModifyDbInstanceShardCapacityResp>,
            tonic::Status,
        >;
        /// ModifyDBInstanceShardNumber
        async fn redis_modify_db_instance_shard_number(
            &self,
            request: tonic::Request<super::RedisModifyDbInstanceShardNumberReq>,
        ) -> std::result::Result<
            tonic::Response<super::RedisModifyDbInstanceShardNumberResp>,
            tonic::Status,
        >;
        /// EnableShardedCluster
        async fn redis_enable_sharded_cluster(
            &self,
            request: tonic::Request<super::RedisEnableShardedClusterReq>,
        ) -> std::result::Result<
            tonic::Response<super::RedisEnableShardedClusterResp>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct RedisInstanceServer<T: RedisInstance> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: RedisInstance> RedisInstanceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RedisInstanceServer<T>
    where
        T: RedisInstance,
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
                "/redis_instance.RedisInstance/RedisDescribeDBInstances" => {
                    #[allow(non_camel_case_types)]
                    struct RedisDescribeDBInstancesSvc<T: RedisInstance>(pub Arc<T>);
                    impl<
                        T: RedisInstance,
                    > tonic::server::UnaryService<super::RedisDescribeDbInstancesReq>
                    for RedisDescribeDBInstancesSvc<T> {
                        type Response = super::RedisDescribeDbInstancesResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RedisDescribeDbInstancesReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).redis_describe_db_instances(request).await
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
                        let method = RedisDescribeDBInstancesSvc(inner);
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
                "/redis_instance.RedisInstance/RedisCreateDBInstance" => {
                    #[allow(non_camel_case_types)]
                    struct RedisCreateDBInstanceSvc<T: RedisInstance>(pub Arc<T>);
                    impl<
                        T: RedisInstance,
                    > tonic::server::UnaryService<super::RedisCreateDbInstanceReq>
                    for RedisCreateDBInstanceSvc<T> {
                        type Response = super::RedisCreateDbInstanceResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RedisCreateDbInstanceReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).redis_create_db_instance(request).await
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
                        let method = RedisCreateDBInstanceSvc(inner);
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
                "/redis_instance.RedisInstance/RedisDescribeDBInstanceDetail" => {
                    #[allow(non_camel_case_types)]
                    struct RedisDescribeDBInstanceDetailSvc<T: RedisInstance>(
                        pub Arc<T>,
                    );
                    impl<
                        T: RedisInstance,
                    > tonic::server::UnaryService<
                        super::RedisDescribeDbInstanceDetailReq,
                    > for RedisDescribeDBInstanceDetailSvc<T> {
                        type Response = super::RedisDescribeDbInstanceDetailResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RedisDescribeDbInstanceDetailReq,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).redis_describe_db_instance_detail(request).await
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
                        let method = RedisDescribeDBInstanceDetailSvc(inner);
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
                "/redis_instance.RedisInstance/RedisIncreaseDBInstanceNodeNumber" => {
                    #[allow(non_camel_case_types)]
                    struct RedisIncreaseDBInstanceNodeNumberSvc<T: RedisInstance>(
                        pub Arc<T>,
                    );
                    impl<
                        T: RedisInstance,
                    > tonic::server::UnaryService<
                        super::RedisIncreaseDbInstanceNodeNumberReq,
                    > for RedisIncreaseDBInstanceNodeNumberSvc<T> {
                        type Response = super::RedisIncreaseDbInstanceNodeNumberResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RedisIncreaseDbInstanceNodeNumberReq,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .redis_increase_db_instance_node_number(request)
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
                        let method = RedisIncreaseDBInstanceNodeNumberSvc(inner);
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
                "/redis_instance.RedisInstance/RedisDecreaseDBInstanceNodeNumber" => {
                    #[allow(non_camel_case_types)]
                    struct RedisDecreaseDBInstanceNodeNumberSvc<T: RedisInstance>(
                        pub Arc<T>,
                    );
                    impl<
                        T: RedisInstance,
                    > tonic::server::UnaryService<
                        super::RedisDecreaseDbInstanceNodeNumberReq,
                    > for RedisDecreaseDBInstanceNodeNumberSvc<T> {
                        type Response = super::RedisDecreaseDbInstanceNodeNumberResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RedisDecreaseDbInstanceNodeNumberReq,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .redis_decrease_db_instance_node_number(request)
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
                        let method = RedisDecreaseDBInstanceNodeNumberSvc(inner);
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
                "/redis_instance.RedisInstance/RedisModifyDBInstanceShardCapacity" => {
                    #[allow(non_camel_case_types)]
                    struct RedisModifyDBInstanceShardCapacitySvc<T: RedisInstance>(
                        pub Arc<T>,
                    );
                    impl<
                        T: RedisInstance,
                    > tonic::server::UnaryService<
                        super::RedisModifyDbInstanceShardCapacityReq,
                    > for RedisModifyDBInstanceShardCapacitySvc<T> {
                        type Response = super::RedisModifyDbInstanceShardCapacityResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RedisModifyDbInstanceShardCapacityReq,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .redis_modify_db_instance_shard_capacity(request)
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
                        let method = RedisModifyDBInstanceShardCapacitySvc(inner);
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
                "/redis_instance.RedisInstance/RedisModifyDBInstanceShardNumber" => {
                    #[allow(non_camel_case_types)]
                    struct RedisModifyDBInstanceShardNumberSvc<T: RedisInstance>(
                        pub Arc<T>,
                    );
                    impl<
                        T: RedisInstance,
                    > tonic::server::UnaryService<
                        super::RedisModifyDbInstanceShardNumberReq,
                    > for RedisModifyDBInstanceShardNumberSvc<T> {
                        type Response = super::RedisModifyDbInstanceShardNumberResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RedisModifyDbInstanceShardNumberReq,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .redis_modify_db_instance_shard_number(request)
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
                        let method = RedisModifyDBInstanceShardNumberSvc(inner);
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
                "/redis_instance.RedisInstance/RedisEnableShardedCluster" => {
                    #[allow(non_camel_case_types)]
                    struct RedisEnableShardedClusterSvc<T: RedisInstance>(pub Arc<T>);
                    impl<
                        T: RedisInstance,
                    > tonic::server::UnaryService<super::RedisEnableShardedClusterReq>
                    for RedisEnableShardedClusterSvc<T> {
                        type Response = super::RedisEnableShardedClusterResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RedisEnableShardedClusterReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).redis_enable_sharded_cluster(request).await
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
                        let method = RedisEnableShardedClusterSvc(inner);
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
    impl<T: RedisInstance> Clone for RedisInstanceServer<T> {
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
    impl<T: RedisInstance> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RedisInstance> tonic::server::NamedService for RedisInstanceServer<T> {
        const NAME: &'static str = "redis_instance.RedisInstance";
    }
}
