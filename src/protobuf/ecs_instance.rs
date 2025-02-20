#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyInstanceSpecReq {
    #[prost(string, optional, tag = "1")]
    pub client_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub dry_run: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "3")]
    pub instance_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub instance_type_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyInstanceSpecResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<ModifyInstanceSpecResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyInstanceSpecResultResp {
    #[prost(string, optional, tag = "1")]
    pub instance_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub order_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeInstancesReq {
    #[prost(string, optional, tag = "1")]
    pub dedicated_host_cluster_id: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "2")]
    pub dedicated_host_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub hpc_cluster_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub instance_charge_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub instance_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub key_pair_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "7")]
    pub max_results: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "8")]
    pub next_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub primary_ip_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub project_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub scheduled_instance_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "13")]
    pub vpc_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "14")]
    pub zone_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "15")]
    pub instance_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, repeated, tag = "16")]
    pub deployment_set_group_numbers: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "17")]
    pub deployment_set_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "18")]
    pub eip_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "19")]
    pub instance_type_families: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "20")]
    pub instance_type_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "21")]
    pub ipv6_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeInstancesTagFiltersReq {
    #[prost(string, optional, tag = "1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeInstancesResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<DescribeInstancesResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeInstancesResultResp {
    #[prost(string, optional, tag = "1")]
    pub next_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub instances: ::prost::alloc::vec::Vec<DescribeInstancesResultInstanceResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeInstancesResultInstanceResp {
    #[prost(int64, optional, tag = "1")]
    pub cpus: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub created_at: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "3")]
    pub deployment_set_group_number: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "4")]
    pub deployment_set_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub elastic_scheduled_instance_type: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "7")]
    pub expired_at: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub hostname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub hpc_cluster_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub image_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub instance_charge_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub instance_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "13")]
    pub instance_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "14")]
    pub instance_type_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "15")]
    pub key_pair_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "16")]
    pub key_pair_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "17")]
    pub memory_size: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "18")]
    pub os_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "19")]
    pub os_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "20")]
    pub project_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "21")]
    pub scheduled_instance_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(float, optional, tag = "22")]
    pub spot_price_limit: ::core::option::Option<f32>,
    #[prost(string, optional, tag = "23")]
    pub spot_strategy: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "24")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "25")]
    pub stopped_mode: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "26")]
    pub updated_at: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "27")]
    pub uuid: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "28")]
    pub vpc_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "29")]
    pub zone_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "30")]
    pub rdma_ip_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "31")]
    pub placement: ::core::option::Option<DescribeInstancesResultInstancePlacementResp>,
    #[prost(message, optional, tag = "32")]
    pub eip_address: ::core::option::Option<
        DescribeInstancesResultInstanceEipAddressResp,
    >,
    #[prost(message, optional, tag = "33")]
    pub cpu_options: ::core::option::Option<
        DescribeInstancesResultInstanceCpuOptionsResp,
    >,
    #[prost(message, repeated, tag = "34")]
    pub local_volumes: ::prost::alloc::vec::Vec<
        DescribeInstancesResultInstanceLocalVolumesResp,
    >,
    #[prost(message, repeated, tag = "35")]
    pub tags: ::prost::alloc::vec::Vec<DescribeInstancesResultInstanceTagResp>,
    #[prost(message, repeated, tag = "36")]
    pub network_interfaces: ::prost::alloc::vec::Vec<
        DescribeInstancesResultInstanceNetworkInterfacesResp,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeInstancesResultInstanceTagResp {
    #[prost(string, optional, tag = "1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeInstancesResultInstancePlacementResp {
    #[prost(string, optional, tag = "1")]
    pub affinity: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub dedicated_host_cluster_id: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "3")]
    pub dedicated_host_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub tenancy: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeInstancesResultInstanceNetworkInterfacesResp {
    #[prost(string, repeated, tag = "1")]
    pub ipv6_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub mac_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub network_interface_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub primary_ip_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub subnet_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub vpc_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeInstancesResultInstanceLocalVolumesResp {
    #[prost(int64, optional, tag = "1")]
    pub count: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub size: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "3")]
    pub volume_type: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeInstancesResultInstanceEipAddressResp {
    #[prost(string, optional, tag = "1")]
    pub allocation_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub ip_address: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeInstancesResultInstanceCpuOptionsResp {
    #[prost(int64, optional, tag = "1")]
    pub core_count: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub threads_per_core: ::core::option::Option<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopInstancesReq {
    #[prost(string, optional, tag = "1")]
    pub client_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub dry_run: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub force_stop: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "4")]
    pub stopped_mode: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "5")]
    pub instance_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopInstancesResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<StopInstancesResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopInstancesResultResp {
    #[prost(message, repeated, tag = "1")]
    pub operation_details: ::prost::alloc::vec::Vec<
        StopInstancesResultOperationDetailsResp,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopInstancesResultOperationDetailsResp {
    #[prost(string, optional, tag = "1")]
    pub instance_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<StopInstancesResultOperationDetailsErrResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopInstancesResultOperationDetailsErrResp {
    #[prost(string, optional, tag = "1")]
    pub code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopInstanceReq {
    #[prost(string, optional, tag = "1")]
    pub client_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub dry_run: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub force_stop: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "4")]
    pub instance_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub stopped_mode: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopInstanceResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<StopInstanceResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopInstanceResultResp {}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunInstancesReq {
    #[prost(bool, optional, tag = "1")]
    pub auto_renew: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "2")]
    pub auto_renew_period: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "3")]
    pub client_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "4")]
    pub count: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "5")]
    pub credit_specification: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "6")]
    pub deployment_set_group_number: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "7")]
    pub deployment_set_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "9")]
    pub dry_run: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "10")]
    pub hostname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub hpc_cluster_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub image_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "13")]
    pub install_run_command_agent: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "14")]
    pub instance_charge_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "15")]
    pub instance_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "16")]
    pub instance_type_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "17")]
    pub keep_image_credential: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "18")]
    pub key_pair_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "19")]
    pub min_count: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "20")]
    pub password: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "21")]
    pub period: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "22")]
    pub period_unit: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "23")]
    pub project_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "24")]
    pub security_enhancement_strategy: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(float, optional, tag = "25")]
    pub spot_price_limit: ::core::option::Option<f32>,
    #[prost(string, optional, tag = "26")]
    pub spot_strategy: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "27")]
    pub suffix_index: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "28")]
    pub unique_suffix: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "29")]
    pub user_data: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "30")]
    pub zone_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "31")]
    pub placement: ::core::option::Option<RunInstancesPlacementReq>,
    #[prost(message, repeated, tag = "32")]
    pub volumes: ::prost::alloc::vec::Vec<RunInstancesVolumesReq>,
    #[prost(message, repeated, tag = "33")]
    pub tags: ::prost::alloc::vec::Vec<RunInstancesTagsReq>,
    #[prost(message, repeated, tag = "34")]
    pub network_interfaces: ::prost::alloc::vec::Vec<RunInstancesNetworkInterfacesReq>,
    #[prost(message, repeated, tag = "35")]
    pub eip_addres: ::prost::alloc::vec::Vec<RunInstancesEipAddressReq>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunInstancesVolumesReq {
    #[prost(int64, optional, tag = "1")]
    pub size: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub extra_performance_iops: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "3")]
    pub extra_performance_throughput_mb: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "4")]
    pub extra_performance_type_id: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "5")]
    pub snapshot_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub volume_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "7")]
    pub delete_with_instance: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunInstancesTagsReq {
    #[prost(string, optional, tag = "1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunInstancesPlacementReq {
    #[prost(string, optional, tag = "1")]
    pub affinity: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub dedicated_host_cluster_id: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "3")]
    pub dedicated_host_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub tenancy: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunInstancesNetworkInterfacesReq {
    #[prost(string, repeated, tag = "1")]
    pub security_group_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub subnet_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, repeated, tag = "3")]
    pub ipv6_address_count: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "4")]
    pub primary_ip_address: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "5")]
    pub private_ip_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunInstancesEipAddressReq {
    #[prost(int64, optional, tag = "1")]
    pub bandwidth_mbps: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub bandwidth_package_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub charge_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub isp: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "5")]
    pub release_with_instance: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "6")]
    pub security_protection_instance_id: ::core::option::Option<i64>,
    #[prost(string, repeated, tag = "7")]
    pub security_protection_types: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunInstancesResp {
    #[prost(message, optional, tag = "1")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<RunInstancesResultResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunInstancesResultResp {
    #[prost(string, repeated, tag = "1")]
    pub instance_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
pub mod ecs_instance_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct EcsInstanceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EcsInstanceClient<tonic::transport::Channel> {
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
    impl<T> EcsInstanceClient<T>
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
        ) -> EcsInstanceClient<InterceptedService<T, F>>
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
            EcsInstanceClient::new(InterceptedService::new(inner, interceptor))
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
        /// runInstances
        pub async fn run_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::RunInstancesReq>,
        ) -> std::result::Result<
            tonic::Response<super::RunInstancesResp>,
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
                "/ecs_instance.EcsInstance/RunInstances",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ecs_instance.EcsInstance", "RunInstances"));
            self.inner.unary(req, path, codec).await
        }
        /// DescribeInstances
        pub async fn describe_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::DescribeInstancesReq>,
        ) -> std::result::Result<
            tonic::Response<super::DescribeInstancesResp>,
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
                "/ecs_instance.EcsInstance/DescribeInstances",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ecs_instance.EcsInstance", "DescribeInstances"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// stopInstance
        pub async fn stop_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::StopInstanceReq>,
        ) -> std::result::Result<
            tonic::Response<super::StopInstanceResp>,
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
                "/ecs_instance.EcsInstance/StopInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ecs_instance.EcsInstance", "StopInstance"));
            self.inner.unary(req, path, codec).await
        }
        /// StopInstances
        pub async fn stop_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::StopInstancesReq>,
        ) -> std::result::Result<
            tonic::Response<super::StopInstancesResp>,
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
                "/ecs_instance.EcsInstance/StopInstances",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ecs_instance.EcsInstance", "StopInstances"));
            self.inner.unary(req, path, codec).await
        }
        /// ModifyInstanceSpec
        pub async fn modify_instance_spec(
            &mut self,
            request: impl tonic::IntoRequest<super::ModifyInstanceSpecReq>,
        ) -> std::result::Result<
            tonic::Response<super::ModifyInstanceSpecResp>,
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
                "/ecs_instance.EcsInstance/ModifyInstanceSpec",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ecs_instance.EcsInstance", "ModifyInstanceSpec"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod ecs_instance_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with EcsInstanceServer.
    #[async_trait]
    pub trait EcsInstance: Send + Sync + 'static {
        /// runInstances
        async fn run_instances(
            &self,
            request: tonic::Request<super::RunInstancesReq>,
        ) -> std::result::Result<
            tonic::Response<super::RunInstancesResp>,
            tonic::Status,
        >;
        /// DescribeInstances
        async fn describe_instances(
            &self,
            request: tonic::Request<super::DescribeInstancesReq>,
        ) -> std::result::Result<
            tonic::Response<super::DescribeInstancesResp>,
            tonic::Status,
        >;
        /// stopInstance
        async fn stop_instance(
            &self,
            request: tonic::Request<super::StopInstanceReq>,
        ) -> std::result::Result<
            tonic::Response<super::StopInstanceResp>,
            tonic::Status,
        >;
        /// StopInstances
        async fn stop_instances(
            &self,
            request: tonic::Request<super::StopInstancesReq>,
        ) -> std::result::Result<
            tonic::Response<super::StopInstancesResp>,
            tonic::Status,
        >;
        /// ModifyInstanceSpec
        async fn modify_instance_spec(
            &self,
            request: tonic::Request<super::ModifyInstanceSpecReq>,
        ) -> std::result::Result<
            tonic::Response<super::ModifyInstanceSpecResp>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct EcsInstanceServer<T: EcsInstance> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: EcsInstance> EcsInstanceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for EcsInstanceServer<T>
    where
        T: EcsInstance,
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
                "/ecs_instance.EcsInstance/RunInstances" => {
                    #[allow(non_camel_case_types)]
                    struct RunInstancesSvc<T: EcsInstance>(pub Arc<T>);
                    impl<
                        T: EcsInstance,
                    > tonic::server::UnaryService<super::RunInstancesReq>
                    for RunInstancesSvc<T> {
                        type Response = super::RunInstancesResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RunInstancesReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).run_instances(request).await
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
                        let method = RunInstancesSvc(inner);
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
                "/ecs_instance.EcsInstance/DescribeInstances" => {
                    #[allow(non_camel_case_types)]
                    struct DescribeInstancesSvc<T: EcsInstance>(pub Arc<T>);
                    impl<
                        T: EcsInstance,
                    > tonic::server::UnaryService<super::DescribeInstancesReq>
                    for DescribeInstancesSvc<T> {
                        type Response = super::DescribeInstancesResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DescribeInstancesReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).describe_instances(request).await
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
                        let method = DescribeInstancesSvc(inner);
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
                "/ecs_instance.EcsInstance/StopInstance" => {
                    #[allow(non_camel_case_types)]
                    struct StopInstanceSvc<T: EcsInstance>(pub Arc<T>);
                    impl<
                        T: EcsInstance,
                    > tonic::server::UnaryService<super::StopInstanceReq>
                    for StopInstanceSvc<T> {
                        type Response = super::StopInstanceResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopInstanceReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).stop_instance(request).await
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
                        let method = StopInstanceSvc(inner);
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
                "/ecs_instance.EcsInstance/StopInstances" => {
                    #[allow(non_camel_case_types)]
                    struct StopInstancesSvc<T: EcsInstance>(pub Arc<T>);
                    impl<
                        T: EcsInstance,
                    > tonic::server::UnaryService<super::StopInstancesReq>
                    for StopInstancesSvc<T> {
                        type Response = super::StopInstancesResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopInstancesReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).stop_instances(request).await
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
                        let method = StopInstancesSvc(inner);
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
                "/ecs_instance.EcsInstance/ModifyInstanceSpec" => {
                    #[allow(non_camel_case_types)]
                    struct ModifyInstanceSpecSvc<T: EcsInstance>(pub Arc<T>);
                    impl<
                        T: EcsInstance,
                    > tonic::server::UnaryService<super::ModifyInstanceSpecReq>
                    for ModifyInstanceSpecSvc<T> {
                        type Response = super::ModifyInstanceSpecResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModifyInstanceSpecReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).modify_instance_spec(request).await
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
                        let method = ModifyInstanceSpecSvc(inner);
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
    impl<T: EcsInstance> Clone for EcsInstanceServer<T> {
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
    impl<T: EcsInstance> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: EcsInstance> tonic::server::NamedService for EcsInstanceServer<T> {
        const NAME: &'static str = "ecs_instance.EcsInstance";
    }
}
