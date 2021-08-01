use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use super::HxMeta;

use super::local::LocalFS;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum HxService {
    
}