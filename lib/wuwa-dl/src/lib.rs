pub mod prelude {
    pub use crate::{
        cli::Cli,
        helper::{ResourceHelperExt, resource::ResourceHelper},
        json::{index::IndexJson, resource::ResourceJson},
        pool::{Pool, PoolOp},
        utils::{AsBoolean, Boolean, DynResult, INDEX_JSON_URL},
    };

    pub(crate) use std::{
        error::Error, fmt::Write, fs::File, io, ops::Not, path::Path, path::PathBuf, time::Duration,
    };

    pub(crate) use tokio::{
        fs::{self, File as AsyncFile},
        io::AsyncWriteExt,
        runtime::Handle,
        sync::{
            mpsc::{self, Sender},
            watch::{self, Receiver},
        },
    };

    pub(crate) use clap::{ArgAction::SetTrue, Parser};
    pub(crate) use futures_util::StreamExt;
    pub(crate) use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
    pub(crate) use md5::{Digest, Md5};
    pub(crate) use serde::{Deserialize, Serialize};
    pub(crate) use serde_json::Value;
}

pub mod private {
    pub use crate::{
        helper::ResourceHelperBase, json::resource::Resource, utils::PROGRESS_STYLE,
        utils::Volatile,
    };
}

mod cli;
mod helper;
mod json;
mod pool;

#[macro_use]
mod utils;

#[cfg(test)]
mod tests;
