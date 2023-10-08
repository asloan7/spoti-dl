use std::collections::HashSet;
use std::path::PathBuf;

use crate::metadata;

use lazy_static::lazy_static;

#[derive(Debug)]
pub struct CliArgs {
    pub download_dir: PathBuf,
    pub codec: metadata::Codec,
    pub bitrate: metadata::Bitrate,
    pub parallel_downloads: u32,
}

pub const INTERNAL_ERROR_MSG: &str =
    "Something went wrong! Please share the log file (located at ~/.spotidl.log) on the \
spoti-dl Github repo (https://github.com/dhruv-ahuja/spoti-dl)";

lazy_static! {
    pub static ref ILLEGAL_PATH_CHARS: HashSet<char> =
        ['\\', '?', '%', '*', ':', '|', '"', '<', '>', '.']
            .iter()
            .cloned()
            .collect();
}
