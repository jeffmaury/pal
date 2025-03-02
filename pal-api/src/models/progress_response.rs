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

/// ProgressResponse : The response returned from various streaming endpoints
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProgressResponse {
    /// The status of the request
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The SHA256 digest of the blob
    #[serde(rename = "digest", skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// The total size of the task
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// The completed size of the task
    #[serde(rename = "completed", skip_serializing_if = "Option::is_none")]
    pub completed: Option<i32>,
}

impl ProgressResponse {
    /// The response returned from various streaming endpoints
    pub fn new() -> ProgressResponse {
        ProgressResponse {
            status: None,
            digest: None,
            total: None,
            completed: None,
        }
    }
}

