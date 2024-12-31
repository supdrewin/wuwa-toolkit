use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceJson {
    pub resource: Vec<Resource>,
    pub sample_hash_info: SampleHashInfo,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub dest: String,
    pub md5: String,
    pub sample_hash: String,
    pub size: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SampleHashInfo {
    pub sample_num: Value, // TODO
    pub sample_block_max_size: u64,
}
