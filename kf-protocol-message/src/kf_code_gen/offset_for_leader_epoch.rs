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
// KfOffsetForLeaderEpochRequest
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfOffsetForLeaderEpochRequest {
    /// Each topic to get offsets for.
    pub topics: Vec<OffsetForLeaderTopic>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct OffsetForLeaderTopic {
    /// The topic name.
    pub name: String,

    /// Each partition to get offsets for.
    pub partitions: Vec<OffsetForLeaderPartition>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct OffsetForLeaderPartition {
    /// The partition index.
    pub partition_index: i32,

    /// An epoch used to fence consumers/replicas with old metadata.  If the epoch provided by the
    /// client is larger than the current epoch known to the broker, then the UNKNOWN_LEADER_EPOCH
    /// error code will be returned. If the provided epoch is smaller, then the FENCED_LEADER_EPOCH
    /// error code will be returned.
    #[fluvio_kf(min_version = 2, ignorable)]
    pub current_leader_epoch: i32,

    /// The epoch to look up an offset for.
    pub leader_epoch: i32,
}

// -----------------------------------
// KfOffsetForLeaderEpochResponse
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfOffsetForLeaderEpochResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation,
    /// or zero if the request did not violate any quota.
    #[fluvio_kf(min_version = 2, ignorable)]
    pub throttle_time_ms: i32,

    /// Each topic we fetched offsets for.
    pub topics: Vec<OffsetForLeaderTopicResult>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct OffsetForLeaderTopicResult {
    /// The topic name.
    pub name: String,

    /// Each partition in the topic we fetched offsets for.
    pub partitions: Vec<OffsetForLeaderPartitionResult>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct OffsetForLeaderPartitionResult {
    /// The error code 0, or if there was no error.
    pub error_code: ErrorCode,

    /// The partition index.
    pub partition_index: i32,

    /// The leader epoch of the partition.
    #[fluvio_kf(min_version = 1, ignorable)]
    pub leader_epoch: i32,

    /// The end offset of the epoch.
    pub end_offset: i64,
}

// -----------------------------------
// Implementation - KfOffsetForLeaderEpochRequest
// -----------------------------------

impl Request for KfOffsetForLeaderEpochRequest {
    const API_KEY: u16 = 23;

    const MIN_API_VERSION: i16 = 0;
    const MAX_API_VERSION: i16 = 2;
    const DEFAULT_API_VERSION: i16 = 2;

    type Response = KfOffsetForLeaderEpochResponse;
}
