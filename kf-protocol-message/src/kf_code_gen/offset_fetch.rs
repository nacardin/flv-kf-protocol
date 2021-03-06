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
// KfOffsetFetchRequest
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfOffsetFetchRequest {
    /// The group to fetch offsets for.
    pub group_id: String,

    /// Each topic we would like to fetch offsets for, or null to fetch offsets for all topics.
    pub topics: Option<Vec<OffsetFetchRequestTopic>>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct OffsetFetchRequestTopic {
    pub name: String,

    /// The partition indexes we would like to fetch offsets for.
    pub partition_indexes: Vec<i32>,
}

// -----------------------------------
// KfOffsetFetchResponse
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfOffsetFetchResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation,
    /// or zero if the request did not violate any quota.
    #[fluvio_kf(min_version = 3, ignorable)]
    pub throttle_time_ms: i32,

    /// The responses per topic.
    pub topics: Vec<OffsetFetchResponseTopic>,

    /// The top-level error code, or 0 if there was no error.
    #[fluvio_kf(min_version = 2)]
    pub error_code: ErrorCode,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct OffsetFetchResponseTopic {
    /// The topic name.
    pub name: String,

    /// The responses per partition
    pub partitions: Vec<OffsetFetchResponsePartition>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct OffsetFetchResponsePartition {
    /// The partition index.
    pub partition_index: i32,

    /// The committed message offset.
    pub committed_offset: i64,

    /// The leader epoch.
    #[fluvio_kf(min_version = 5)]
    pub committed_leader_epoch: i32,

    /// The partition metadata.
    pub metadata: Option<String>,

    /// The error code, or 0 if there was no error.
    pub error_code: ErrorCode,
}

// -----------------------------------
// Implementation - KfOffsetFetchRequest
// -----------------------------------

impl Request for KfOffsetFetchRequest {
    const API_KEY: u16 = 9;

    const MIN_API_VERSION: i16 = 0;
    const MAX_API_VERSION: i16 = 5;
    const DEFAULT_API_VERSION: i16 = 5;

    type Response = KfOffsetFetchResponse;
}
