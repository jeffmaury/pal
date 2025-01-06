/*
 * Podman Desktop AI Lab API
 *
 * API for interacting with the Podman Desktop AI Lab service.
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetServerVersion200Response {
    #[serde(rename = "version")]
    pub version: String,
}

impl GetServerVersion200Response {
    pub fn new(version: String) -> GetServerVersion200Response {
        GetServerVersion200Response {
            version,
        }
    }
}

