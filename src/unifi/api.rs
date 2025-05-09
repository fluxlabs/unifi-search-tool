
#![allow(dead_code)]

use super::devices::{ClientDevice, ClientDeviceActive, UnifiDeviceBasic, UnifiSite};
use reqwest::{blocking::Client, header::REFERER, StatusCode};
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;
use zeroize::Zeroize;

#[derive(Debug, Clone, Deserialize)]
struct RespMeta {
    #[serde(rename(deserialize = "rc"))]
    result: RespResult,
    msg: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
enum RespResult {
    Ok,
    Error,
}

#[derive(Debug, Clone, Deserialize)]
struct UnifiSitesResp {
    meta: RespMeta,
    data: Vec<UnifiSite>,
}

#[derive(Debug, Clone, Deserialize)]
struct UnifiDevicesBasicResp {
    meta: RespMeta,
    data: Vec<UnifiDeviceBasic>,
}

#[derive(Debug, Clone, Deserialize)]
struct UnifiClientsAllResp {
    meta: RespMeta,
    data: Vec<ClientDevice>,
}

#[derive(Debug, Clone, Deserialize)]
struct UnifiClientsActiveResp {
    meta: RespMeta,
    data: Vec<ClientDeviceActive>,
}

#[derive(Error, Debug)]
pub(crate) enum UnifiAPIError {
    #[error("Failed to create HTTP client")]
    ClientError { source: reqwest::Error },

    #[error("Login failed: invalid credentials for {url}")]
    LoginAuthenticationError { url: String },

    #[error("Request error")]
    ReqwestError { source: reqwest::Error },

    #[error("JSON parsing failed for {url}")]
    JsonError { url: String, source: simd_json::Error },
}

pub(crate) struct UnifiClient<'a> {
    client: Client,
    server_url: &'a str,
    is_logged_in: bool,
}

impl<'a> UnifiClient<'a> {
    pub(crate) fn new(server_url: &'a str, accept_invalid_certs: bool) -> Result<Self, UnifiAPIError> {
        let client = Client::builder()
            .danger_accept_invalid_certs(accept_invalid_certs)
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| UnifiAPIError::ClientError { source: e })?;

        Ok(Self {
            client,
            server_url,
            is_logged_in: false,
        })
    }

    // Additional methods (e.g., login, fetch_sites) would follow here...
}
