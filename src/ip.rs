use anyhow::Result;
use std::net::{Ipv4Addr, Ipv6Addr};

pub async fn get_public_ipv4() -> Result<Option<Ipv4Addr>> {
    match reqwest::get("https://ipv4.icanhazip.com").await {
        Ok(resp) => {
            let text = resp.text().await?;
            let ip = text.trim().parse()?;
            Ok(Some(ip))
        }
        Err(e) => {
            if e.is_connect() {
                // connect error (often) implies there is no ipv4 routing
                Ok(None)
            } else {
                Err(e.into())
            }
        }
    }
}

pub async fn get_public_ipv6() -> Result<Option<Ipv6Addr>> {
    match reqwest::get("https://ipv6.icanhazip.com").await {
        Ok(resp) => {
            let text = resp.text().await?;
            let ip = text.trim().parse()?;
            Ok(Some(ip))
        }
        Err(e) => {
            if e.is_connect() {
                // connect error (often) implies there is no ipv6 routing
                Ok(None)
            } else {
                Err(e.into())
            }
        }
    }
}
