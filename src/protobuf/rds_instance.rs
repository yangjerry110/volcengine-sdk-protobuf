#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstancesReq {
    #[prost(string, optional, tag = "1")]
    pub project_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub instance_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub instance_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub instance_status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub db_engine_version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub create_time_start: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub create_time_end: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub zone_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub charge_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub instance_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "11")]
    pub page_number: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "12")]
    pub node_spec: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "13")]
    pub page_size: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "14")]
    #[serde(default)]
    pub tag_filters: ::prost::alloc::vec::Vec<DescribeDbInstancesTagFilterReq>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstancesTagFilterReq {
    #[prost(string, optional, tag = "1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstancesResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<DescribeDbInstancesResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstancesResultResp {
    #[prost(int64, optional, tag = "1")]
    pub total: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "2")]
    pub instances: ::prost::alloc::vec::Vec<DescribeDbInstancesResultInstanceResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstancesResultInstanceResp {
    #[prost(string, optional, tag = "1")]
    pub allow_list_version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub db_engine_version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub instance_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub instance_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub instance_status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub instance_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub lower_case_table_names: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(float, optional, tag = "9")]
    pub node_cpu_used_percentage: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "10")]
    pub node_memory_used_percentage: ::core::option::Option<f32>,
    #[prost(int64, optional, tag = "11")]
    pub node_number: ::core::option::Option<i64>,
    #[prost(float, optional, tag = "12")]
    pub node_space_used_percentage: ::core::option::Option<f32>,
    #[prost(string, optional, tag = "13")]
    pub node_spec: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "14")]
    pub project_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "15")]
    pub region_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "16")]
    pub storage_space: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "17")]
    pub storage_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "18")]
    pub subnet_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "19")]
    pub time_zone: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "20")]
    pub vpc_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "21")]
    pub zone_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "22")]
    pub charge_detail: ::core::option::Option<
        DescribeDbInstancesResultInstanceChargeDetailResp,
    >,
    #[prost(message, optional, tag = "23")]
    pub maintenance_window: ::core::option::Option<
        DescribeDbInstancesResultInstanceMaintenanceWindowResp,
    >,
    #[prost(string, repeated, tag = "24")]
    pub zone_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "25")]
    pub address_object: ::prost::alloc::vec::Vec<
        DescribeDbInstancesResultInstanceAddressResp,
    >,
    #[prost(message, repeated, tag = "26")]
    #[serde(default)]
    pub tags: ::prost::alloc::vec::Vec<DescribeDbInstancesResultInstanceTagResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstancesResultInstanceTagResp {
    #[prost(string, optional, tag = "1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstancesResultInstanceMaintenanceWindowResp {
    #[prost(string, optional, tag = "1")]
    pub maintenance_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub day_kind: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub day_of_week: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstancesResultInstanceChargeDetailResp {
    #[prost(string, optional, tag = "1")]
    pub charge_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub auto_renew: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "3")]
    pub period_unit: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "4")]
    pub period: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "5")]
    pub charge_status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub charge_start_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub charge_end_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub overdue_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub overdue_reclaim_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub temp_modify_start_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub temp_modify_end_time: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstancesResultInstanceAddressResp {
    #[prost(bool, optional, tag = "1")]
    pub dns_visibility: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "2")]
    pub domain: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub eip_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub ip_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub internet_protocol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub network_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub port: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub subnet_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyDbInstanceSpecReq {
    #[prost(string, optional, tag = "1")]
    pub instance_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub modify_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub storage_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "4")]
    pub storage_space: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "5")]
    pub switch_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "6")]
    pub node_info: ::prost::alloc::vec::Vec<ModifyDbInstanceSpecNodeInfoReq>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyDbInstanceSpecNodeInfoReq {
    #[prost(string, optional, tag = "1")]
    pub node_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub zone_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub node_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub node_spec: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub node_operate_type: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyDbInstanceSpecResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<ModifyDbInstanceSpecResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyDbInstanceSpecResultResp {
    #[prost(string, optional, tag = "1")]
    pub instance_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub order_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstanceDetailReq {
    #[prost(string, optional, tag = "1")]
    pub instance_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstanceDetailResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<DescribeDbInstanceDetailResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstanceDetailResultResp {
    #[prost(message, optional, tag = "1")]
    pub basic_info: ::core::option::Option<DescribeDbInstanceDetailResultBasicInfoResp>,
    #[prost(message, optional, tag = "2")]
    pub charge_detail: ::core::option::Option<
        DescribeDbInstanceDetailResultChargeDetailResp,
    >,
    #[prost(message, repeated, tag = "3")]
    pub endpoints: ::prost::alloc::vec::Vec<DescribeDbInstanceDetailResultEndpointResp>,
    #[prost(message, repeated, tag = "4")]
    pub node_detail_info: ::prost::alloc::vec::Vec<
        DescribeDbInstanceDetailResultNodeResp,
    >,
    #[prost(message, optional, tag = "5")]
    pub proxy_detail: ::core::option::Option<
        DescribeDbInstanceDetailResultProxyDetailResp,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstanceDetailResultProxyDetailResp {
    #[prost(bool, optional, tag = "1")]
    pub charged: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "2")]
    pub db_proxy_status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub proxy_resource_info: ::core::option::Option<
        DescribeDbInstanceDetailResultProxyDetailProxyResourceInfoResp,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstanceDetailResultProxyDetailProxyResourceInfoResp {
    #[prost(int64, optional, tag = "1")]
    pub current_proxy_cpu_num: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub max_proxy_cpu_num: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "3")]
    pub min_proxy_cpu_num: ::core::option::Option<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstanceDetailResultNodeResp {
    #[prost(string, optional, tag = "1")]
    pub instance_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub node_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub region_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub zone_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub node_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub node_status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub node_spec: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "8")]
    pub vcpu: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "9")]
    pub memory: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "10")]
    pub create_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub update_time: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstanceDetailResultChargeDetailResp {
    #[prost(string, optional, tag = "1")]
    pub charge_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub auto_renew: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "3")]
    pub period_unit: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "4")]
    pub period: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "5")]
    pub charge_status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub charge_start_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub charge_end_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub overdue_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub overdue_reclaim_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub temp_modify_start_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub temp_modify_end_time: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstanceDetailResultBasicInfoResp {
    #[prost(string, optional, tag = "1")]
    pub instance_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub instance_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub instance_status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub region_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub zone_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    #[serde(rename = "DBEngine")]
    pub db_engine: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    #[serde(rename = "DBEngineVersion")]
    pub db_engine_version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub instance_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "9")]
    pub vcpu: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "10")]
    pub memory: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "11")]
    pub node_spec: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub node_number: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "13")]
    pub create_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "14")]
    pub update_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "15")]
    pub storage_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "16")]
    pub storage_space: ::core::option::Option<i64>,
    #[prost(float, optional, tag = "17")]
    pub storage_use: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "18")]
    pub backup_use: ::core::option::Option<f32>,
    #[prost(string, optional, tag = "19")]
    pub vpc_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "20")]
    pub subnet_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "21")]
    pub time_zone: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "22")]
    pub lower_case_table_names: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "23")]
    pub data_sync_mode: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "24")]
    pub project_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "25")]
    pub allow_list_version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "26")]
    #[serde(default)]
    pub tags: ::prost::alloc::vec::Vec<DescribeDbInstanceDetailResultBasicInfoTagResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstanceDetailResultEndpointResp {
    #[prost(string, optional, tag = "1")]
    pub endpoint_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub endpoint_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub endpoint_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub read_write_mode: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub auto_add_new_nodes: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub enable_read_write_splitting: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "8")]
    pub enable_read_only: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "9")]
    pub addresses: ::prost::alloc::vec::Vec<
        DescribeDbInstanceDetailResultEndpointAddressesResp,
    >,
    #[prost(message, repeated, tag = "10")]
    pub address: ::prost::alloc::vec::Vec<
        DescribeDbInstanceDetailResultEndpointAddressesResp,
    >,
    #[prost(message, repeated, tag = "11")]
    pub read_only_node_weight: ::prost::alloc::vec::Vec<
        DescribeDbInstanceDetailResultEndpointReadOnlyNodeWeightResp,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstanceDetailResultEndpointReadOnlyNodeWeightResp {
    #[prost(string, optional, tag = "1")]
    pub node_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub node_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "3")]
    pub weight: ::core::option::Option<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstanceDetailResultEndpointAddressResp {
    #[prost(bool, optional, tag = "1")]
    pub dns_visibility: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "2")]
    pub domain: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub eip_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub ip_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub internet_protocol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub network_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub port: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub subnet_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstanceDetailResultEndpointAddressesResp {
    #[prost(bool, optional, tag = "1")]
    pub dns_visibility: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "2")]
    pub domain: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub eip_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub ip_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub internet_protocol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub network_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub port: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub subnet_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDbInstanceDetailResultBasicInfoTagResp {
    #[prost(string, optional, tag = "1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDbInstanceReq {
    #[prost(string, optional, tag = "1")]
    pub db_engine_version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub storage_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "3")]
    pub storage_space: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "4")]
    pub instance_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub vpc_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub subnet_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub instance_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub super_account_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub super_account_password: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub lower_case_table_names: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub db_time_zone: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub db_param_group_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "13")]
    pub project_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "14")]
    pub port: ::core::option::Option<i64>,
    #[prost(message, optional, tag = "15")]
    pub charge_info: ::core::option::Option<CreateDbInstanceChargeInfoReq>,
    #[prost(message, optional, tag = "16")]
    pub maintenance_window: ::core::option::Option<
        CreateDbInstanceMaintenanceWindowObjectReq,
    >,
    #[prost(string, repeated, tag = "17")]
    pub allow_list_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "18")]
    pub node_info: ::prost::alloc::vec::Vec<CreateDbInstanceNodeInfoReq>,
    #[prost(message, repeated, tag = "19")]
    pub instance_tags: ::prost::alloc::vec::Vec<CreateDbInstanceTagReq>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDbInstanceMaintenanceWindowObjectReq {
    #[prost(string, optional, tag = "1")]
    pub maintenance_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub day_kind: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub day_of_week: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDbInstanceTagReq {
    #[prost(string, optional, tag = "1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDbInstanceChargeInfoReq {
    #[prost(string, optional, tag = "1")]
    pub charge_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub auto_renew: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "3")]
    pub period_unit: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "4")]
    pub period: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "5")]
    pub number: ::core::option::Option<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDbInstanceNodeInfoReq {
    #[prost(string, optional, tag = "1")]
    pub node_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub zone_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub node_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub node_spec: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub node_operate_type: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDbInstanceResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<CreateDbInstanceResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDbInstanceResultResp {
    #[prost(string, optional, tag = "1")]
    pub instance_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub order_id: ::core::option::Option<::prost::alloc::string::String>,
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
pub mod rds_instance_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct RdsInstanceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RdsInstanceClient<tonic::transport::Channel> {
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
    impl<T> RdsInstanceClient<T>
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
        ) -> RdsInstanceClient<InterceptedService<T, F>>
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
            RdsInstanceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn describe_db_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::DescribeDbInstancesReq>,
        ) -> std::result::Result<
            tonic::Response<super::DescribeDbInstancesResp>,
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
                "/rds_instance.RdsInstance/DescribeDBInstances",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("rds_instance.RdsInstance", "DescribeDBInstances"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// CreateDBInstance
        pub async fn create_db_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDbInstanceReq>,
        ) -> std::result::Result<
            tonic::Response<super::CreateDbInstanceResp>,
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
                "/rds_instance.RdsInstance/CreateDBInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("rds_instance.RdsInstance", "CreateDBInstance"));
            self.inner.unary(req, path, codec).await
        }
        /// ModifyDBInstanceSpec
        pub async fn modify_db_instance_spec(
            &mut self,
            request: impl tonic::IntoRequest<super::ModifyDbInstanceSpecReq>,
        ) -> std::result::Result<
            tonic::Response<super::ModifyDbInstanceSpecResp>,
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
                "/rds_instance.RdsInstance/ModifyDBInstanceSpec",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("rds_instance.RdsInstance", "ModifyDBInstanceSpec"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DescribeDBInstanceDetail
        pub async fn describe_db_instance_detail(
            &mut self,
            request: impl tonic::IntoRequest<super::DescribeDbInstanceDetailReq>,
        ) -> std::result::Result<
            tonic::Response<super::DescribeDbInstanceDetailResp>,
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
                "/rds_instance.RdsInstance/DescribeDBInstanceDetail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "rds_instance.RdsInstance",
                        "DescribeDBInstanceDetail",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod rds_instance_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RdsInstanceServer.
    #[async_trait]
    pub trait RdsInstance: Send + Sync + 'static {
        /// DescribeDBInstances
        async fn describe_db_instances(
            &self,
            request: tonic::Request<super::DescribeDbInstancesReq>,
        ) -> std::result::Result<
            tonic::Response<super::DescribeDbInstancesResp>,
            tonic::Status,
        >;
        /// CreateDBInstance
        async fn create_db_instance(
            &self,
            request: tonic::Request<super::CreateDbInstanceReq>,
        ) -> std::result::Result<
            tonic::Response<super::CreateDbInstanceResp>,
            tonic::Status,
        >;
        /// ModifyDBInstanceSpec
        async fn modify_db_instance_spec(
            &self,
            request: tonic::Request<super::ModifyDbInstanceSpecReq>,
        ) -> std::result::Result<
            tonic::Response<super::ModifyDbInstanceSpecResp>,
            tonic::Status,
        >;
        /// DescribeDBInstanceDetail
        async fn describe_db_instance_detail(
            &self,
            request: tonic::Request<super::DescribeDbInstanceDetailReq>,
        ) -> std::result::Result<
            tonic::Response<super::DescribeDbInstanceDetailResp>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct RdsInstanceServer<T: RdsInstance> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: RdsInstance> RdsInstanceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RdsInstanceServer<T>
    where
        T: RdsInstance,
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
                "/rds_instance.RdsInstance/DescribeDBInstances" => {
                    #[allow(non_camel_case_types)]
                    struct DescribeDBInstancesSvc<T: RdsInstance>(pub Arc<T>);
                    impl<
                        T: RdsInstance,
                    > tonic::server::UnaryService<super::DescribeDbInstancesReq>
                    for DescribeDBInstancesSvc<T> {
                        type Response = super::DescribeDbInstancesResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DescribeDbInstancesReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).describe_db_instances(request).await
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
                        let method = DescribeDBInstancesSvc(inner);
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
                "/rds_instance.RdsInstance/CreateDBInstance" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDBInstanceSvc<T: RdsInstance>(pub Arc<T>);
                    impl<
                        T: RdsInstance,
                    > tonic::server::UnaryService<super::CreateDbInstanceReq>
                    for CreateDBInstanceSvc<T> {
                        type Response = super::CreateDbInstanceResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDbInstanceReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_db_instance(request).await
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
                        let method = CreateDBInstanceSvc(inner);
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
                "/rds_instance.RdsInstance/ModifyDBInstanceSpec" => {
                    #[allow(non_camel_case_types)]
                    struct ModifyDBInstanceSpecSvc<T: RdsInstance>(pub Arc<T>);
                    impl<
                        T: RdsInstance,
                    > tonic::server::UnaryService<super::ModifyDbInstanceSpecReq>
                    for ModifyDBInstanceSpecSvc<T> {
                        type Response = super::ModifyDbInstanceSpecResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModifyDbInstanceSpecReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).modify_db_instance_spec(request).await
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
                        let method = ModifyDBInstanceSpecSvc(inner);
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
                "/rds_instance.RdsInstance/DescribeDBInstanceDetail" => {
                    #[allow(non_camel_case_types)]
                    struct DescribeDBInstanceDetailSvc<T: RdsInstance>(pub Arc<T>);
                    impl<
                        T: RdsInstance,
                    > tonic::server::UnaryService<super::DescribeDbInstanceDetailReq>
                    for DescribeDBInstanceDetailSvc<T> {
                        type Response = super::DescribeDbInstanceDetailResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DescribeDbInstanceDetailReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).describe_db_instance_detail(request).await
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
                        let method = DescribeDBInstanceDetailSvc(inner);
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
    impl<T: RdsInstance> Clone for RdsInstanceServer<T> {
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
    impl<T: RdsInstance> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RdsInstance> tonic::server::NamedService for RdsInstanceServer<T> {
        const NAME: &'static str = "rds_instance.RdsInstance";
    }
}
