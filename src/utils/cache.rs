//! Cache utilities for KV storage

use worker::kv::KvStore;
use serde::{de::DeserializeOwned, Serialize};

pub async fn get_cached<T: DeserializeOwned>(
    kv: &KvStore,
    key: &str
) -> Option<T> {
    kv.get(key)
        .text()
        .await
        .ok()
        .flatten()
        .and_then(|s| serde_json::from_str(&s).ok())
}

pub async fn set_cached<T: Serialize>(
    kv: &KvStore,
    key: &str,
    value: &T,
    ttl_seconds: u64
) -> Result<(), String> {
    let json = serde_json::to_string(value).map_err(|e| e.to_string())?;
    kv.put(key, json)
        .map_err(|e| e.to_string())?
        .expiration_ttl(ttl_seconds)
        .execute()
        .await
        .map_err(|e| e.to_string())
}
