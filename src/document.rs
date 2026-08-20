use crate::Error;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub const SCHEMA_VERSION: &str = "scenarium.v1";

#[derive(Serialize)]
struct DocumentRef<'a, T> {
    schema: &'static str,
    payload: &'a T,
}

#[derive(Deserialize)]
struct Document<T> {
    schema: String,
    payload: T,
}

pub fn encode_document<T: Serialize>(value: &T) -> Result<String, Error> {
    serde_json::to_string_pretty(&DocumentRef {
        schema: SCHEMA_VERSION,
        payload: value,
    })
    .map_err(|error| Error::Json(error.to_string()))
}

pub fn decode_document<T: DeserializeOwned>(json: &str) -> Result<T, Error> {
    let document: Document<T> =
        serde_json::from_str(json).map_err(|error| Error::Json(error.to_string()))?;
    if document.schema != SCHEMA_VERSION {
        return Err(Error::InvalidSchema {
            expected: SCHEMA_VERSION.to_string(),
            actual: document.schema,
        });
    }
    Ok(document.payload)
}
