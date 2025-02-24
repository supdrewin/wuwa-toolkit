use crate::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexJson {
    pub hash_cache_check_acc_switch: Boolean,
    pub default: Default,
    pub predownload_switch: Boolean,
    #[serde(rename = "RHIOptionSwitch")]
    pub rhi_option_switch: Boolean,
    #[serde(rename = "RHIOptionList")]
    pub rhi_option_list: Vec<RhiOption>,
    pub resources_login: ResourcesLogin,
    pub check_exe_is_running: Boolean,
    pub key_file_check_switch: Boolean,
    pub key_file_check_list: Vec<String>,
    pub chunk_download_switch: Boolean,
    pub fingerprints: Option<Vec<String>>,
    pub resources_gray: Option<ResourcesGray>,
    pub experiment: Experiment,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Default {
    pub cdn_list: Vec<Cdn>,
    pub changelog: Value, // TODO
    pub changelog_visible: Boolean,
    pub resource_chunk: ResourceChunk,
    pub resources: String,
    pub resources_base_path: String,
    pub resources_diff: ResourcesDiff,
    pub resources_exclude_path: Vec<Value>,             // TODO
    pub resources_exclude_path_need_update: Vec<Value>, // TODO
    pub sample_hash_switch: Boolean,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct Cdn {
    #[serde(rename = "K1")]
    pub k1: Value, // TODO
    #[serde(rename = "K2")]
    pub k2: Value, // TODO
    #[serde(rename = "P")]
    pub p: Value, // TODO
    pub url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceChunk {
    pub last_md5: String,
    pub last_resource_chunk_path: String,
    pub last_resources: String,
    pub last_version: String,
    pub md5: String,
    pub resource_chunk_path: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesDiff {
    pub current_game_info: GameInfo,
    pub previous_game_info: GameInfo,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameInfo {
    pub file_name: String,
    pub md5: String,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RhiOption {
    pub cmd_option: String,
    pub is_show: Boolean,
    pub text: Value, // TODO
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesLogin {
    pub host: String,
    pub login_switch: Boolean,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesGray {
    pub gray_switch: Boolean,
}

#[derive(Serialize, Deserialize)]
pub struct Experiment {
    pub download: Download,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Download {
    pub drop_network_error: Boolean,
    pub disabled_compressed: Boolean,
    pub drop_wrong_content_length: Boolean,
    pub drop_wrong_content_encoding: Boolean,
}
