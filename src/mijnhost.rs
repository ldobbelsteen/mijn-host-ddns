use crate::ddns::Record;
use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

const API_BASE_URL: &str = "https://mijn.host/api/v2";

pub async fn get_records(client: &Client, api_key: &str, domain_name: &str) -> Result<Vec<Record>> {
    #[derive(Debug, Deserialize)]
    struct GetRecordResponseData {
        records: Vec<Record>,
    }

    #[derive(Debug, Deserialize)]
    struct GetRecordsResponse {
        data: GetRecordResponseData,
    }

    let url = format!("{API_BASE_URL}/domains/{domain_name}/dns");

    let resp = client
        .get(url)
        .header("API-Key", api_key)
        .send()
        .await?
        .error_for_status()?;

    let parsed: GetRecordsResponse = resp.json().await?;
    Ok(parsed.data.records)
}

pub async fn put_records(
    client: &Client,
    api_key: &str,
    domain_name: &str,
    records: Vec<Record>,
) -> Result<()> {
    let url = format!("{API_BASE_URL}/domains/{domain_name}/dns");

    let mut body = HashMap::new();
    body.insert("records", records);

    client
        .put(url)
        .header("API-Key", api_key)
        .json(&body)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
