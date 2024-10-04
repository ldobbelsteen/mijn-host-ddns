use crate::ddns::Record;
use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, ClientBuilder};
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;

const API_BASE_URL: &str = "https://mijn.host/api/v2";

pub async fn build_client(api_key: &str) -> Result<Client> {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_str("API-Key")?,
        HeaderValue::from_str(api_key)?,
    );
    let client = ClientBuilder::new().default_headers(headers).build()?;
    Ok(client)
}

pub async fn get_records(client: &Client, domain_name: &str) -> Result<Vec<Record>> {
    #[derive(Debug, Deserialize)]
    struct GetRecordResponseData {
        records: Vec<Record>,
    }

    #[derive(Debug, Deserialize)]
    struct GetRecordsResponse {
        data: GetRecordResponseData,
    }

    let url = format!("{API_BASE_URL}/domains/{domain_name}/dns");
    let resp = client.get(url).send().await?.error_for_status()?;
    let parsed: GetRecordsResponse = resp.json().await?;

    Ok(parsed.data.records)
}

pub async fn put_records(client: &Client, domain_name: &str, records: Vec<Record>) -> Result<()> {
    let url = format!("{API_BASE_URL}/domains/{domain_name}/dns");

    let mut body = HashMap::new();
    body.insert("records", records);

    client
        .put(url)
        .json(&body)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
