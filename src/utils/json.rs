package utils;

use serde_json;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn serialize_to_json<T: Serialize>(data: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(data)
}

pub fn deserialize_from_json<T: DeserializeOwned>(json_str: &str) -> Result<T, serde_json::Error> {
    serde_json::from_str(json_str)
}