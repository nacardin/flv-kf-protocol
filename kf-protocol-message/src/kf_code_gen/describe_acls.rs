/// WARNING: CODE GENERATED FILE
/// * This file is generated by kfspec2code.
/// * Any changes applied to this file will be lost when a new spec is generated.
use serde::{Deserialize, Serialize};

use kf_protocol_api::ErrorCode;
use kf_protocol_api::Request;

use kf_protocol_derive::Decode;
use kf_protocol_derive::Encode;
use kf_protocol_derive::KfDefault;

// -----------------------------------
// KfDescribeAclsRequest
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfDescribeAclsRequest {
    /// The resource type.
    pub resource_type: i8,

    /// The resource name, or null to match any resource name.
    pub resource_name_filter: Option<String>,

    /// The resource pattern to match.
    #[fluvio_kf(min_version = 1)]
    pub resource_pattern_type: i8,

    /// The principal to match, or null to match any principal.
    pub principal_filter: Option<String>,

    /// The host to match, or null to match any host.
    pub host_filter: Option<String>,

    /// The operation to match.
    pub operation: i8,

    /// The permission type to match.
    pub permission_type: i8,
}

// -----------------------------------
// KfDescribeAclsResponse
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfDescribeAclsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation,
    /// or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: ErrorCode,

    /// The error message, or null if there was no error.
    pub error_message: Option<String>,

    /// Each Resource that is referenced in an ACL.
    pub resources: Vec<DescribeAclsResource>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct DescribeAclsResource {
    /// The resource type.
    pub typ: i8,

    /// The resource name.
    pub name: String,

    /// The resource pattern type.
    #[fluvio_kf(min_version = 1)]
    pub pattern_type: i8,

    /// The ACLs.
    pub acls: Vec<AclDescription>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct AclDescription {
    /// The ACL principal.
    pub principal: String,

    /// The ACL host.
    pub host: String,

    /// The ACL operation.
    pub operation: i8,

    /// The ACL permission type.
    pub permission_type: i8,
}

// -----------------------------------
// Implementation - KfDescribeAclsRequest
// -----------------------------------

impl Request for KfDescribeAclsRequest {
    const API_KEY: u16 = 29;

    const MIN_API_VERSION: i16 = 0;
    const MAX_API_VERSION: i16 = 1;
    const DEFAULT_API_VERSION: i16 = 1;

    type Response = KfDescribeAclsResponse;
}
