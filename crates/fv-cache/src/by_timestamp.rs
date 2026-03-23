use fv_cache_info::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CachedByTimestamp<Data> {
    pub timestamp: Timestamp,
    pub data: Data,
}
