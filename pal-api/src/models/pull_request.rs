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

/// PullRequest : Request to pull a model
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PullRequest {
    /// The name of the model to pull
    #[serde(rename = "model")]
    pub model: String,
    /// allow insecure connections to the catalog. 
    #[serde(rename = "insecure", skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    /// If false the response will be returned as a single response object, rather than a stream of objects 
    #[serde(rename = "stream", skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

impl PullRequest {
    /// Request to pull a model
    pub fn new(model: String) -> PullRequest {
        PullRequest {
            model,
            insecure: None,
            stream: None,
        }
    }
}

