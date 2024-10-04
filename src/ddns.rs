use crate::{
    ip::{get_public_ipv4, get_public_ipv6},
    mijnhost::{get_records, put_records},
    Config,
};
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

const DEFAULT_TTL: u64 = 3600;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Record {
    pub r#type: String,
    pub name: String,
    pub value: String,
    pub ttl: u64,
}

pub async fn routine(config: &Config, client: &Client) -> Result<()> {
    log::info!("running update routine...");

    let mut existing_records = get_records(client, &config.domain_name).await?;
    let action_taken = update_record_list(&mut existing_records, config).await?;

    if action_taken {
        log::debug!("putting records back to the API...");
        put_records(client, &config.domain_name, existing_records).await?;
    } else {
        log::info!("no action required...");
    }

    Ok(())
}

/// Given a list of existing records, find the A and AAAA records and update them if necessary
/// (based on the public IPv4 and IPv6 addresses available). The modification is in-place in
/// the records vector. Returns whether any action was taken.
#[allow(clippy::too_many_lines)]
async fn update_record_list(records: &mut Vec<Record>, config: &Config) -> Result<bool> {
    let a_idx = records
        .iter()
        .position(|r| r.r#type == "A" && r.name == config.record_name);

    let aaaa_idx = records
        .iter()
        .position(|r| r.r#type == "AAAA" && r.name == config.record_name);

    let mut action_taken = false;
    let mut remove_a_rec = false;
    let mut remove_aaaa_rec = false;

    match a_idx {
        Some(i) => {
            let a_rec = &mut records[i];
            if let Some(ipv4) = get_public_ipv4().await? {
                if ipv4 == Ipv4Addr::from_str(&a_rec.value)? {
                    log::debug!("public ipv4 found ({}) which matches the A record...", ipv4);
                } else {
                    a_rec.value = ipv4.to_string();
                    log::info!("A record IP updated to {}...", ipv4);
                    action_taken = true;
                }
            } else if config.manage_records {
                remove_a_rec = true;
            } else {
                log::warn!(
                    "public ipv4 not found but an A record ({}) exists, consider enabling record management",
                    a_rec.value
                );
            }
        }
        None => {
            if let Some(ipv4) = get_public_ipv4().await? {
                if config.manage_records {
                    let new_record = Record {
                        r#type: "A".into(),
                        name: config.record_name.clone(),
                        value: ipv4.to_string(),
                        ttl: match aaaa_idx {
                            Some(i) => records[i].ttl,
                            None => DEFAULT_TTL,
                        },
                    };
                    log::info!(
                        "A record created with IP {} and a TTL of {} seconds...",
                        new_record.value,
                        new_record.ttl,
                    );
                    records.push(new_record);
                    action_taken = true;
                } else {
                    log::warn!(
                        "public ipv4 found ({}) but no A record exists, consider enabling record management",
                        ipv4
                    );
                }
            } else {
                log::debug!("public ipv4 not found, matching the absence of an A record...");
            }
        }
    }

    match aaaa_idx {
        Some(i) => {
            let aaaa_rec = &mut records[i];
            if let Some(ipv6) = get_public_ipv6().await? {
                if ipv6 == Ipv6Addr::from_str(&aaaa_rec.value)? {
                    log::debug!(
                        "public ipv6 found ({}) which matches the AAAA record...",
                        ipv6
                    );
                } else {
                    aaaa_rec.value = ipv6.to_string();
                    log::info!("AAAA record IP updated to {}...", ipv6);
                    action_taken = true;
                }
            } else if config.manage_records {
                remove_aaaa_rec = true;
            } else {
                log::warn!(
                    "public ipv6 not found but an AAAA record ({}) exists, consider enabling record management",
                    aaaa_rec.value
                );
            }
        }
        None => {
            if let Some(ipv6) = get_public_ipv6().await? {
                if config.manage_records {
                    let new_record = Record {
                        r#type: "AAAA".into(),
                        name: config.record_name.clone(),
                        value: ipv6.to_string(),
                        ttl: match a_idx {
                            Some(i) => records[i].ttl,
                            None => DEFAULT_TTL,
                        },
                    };
                    log::info!(
                        "AAAA record created with IP {} and a TTL of {} seconds...",
                        new_record.value,
                        new_record.ttl,
                    );
                    records.push(new_record);
                    action_taken = true;
                } else {
                    log::warn!(
                        "public ipv6 found ({}) but no AAAA record exists, consider enabling record management",
                        ipv6
                    );
                }
            } else {
                log::debug!("public ipv6 not found, matching the absence of an AAAA record...");
            }
        }
    }

    if remove_a_rec {
        records.retain(|r| !(r.r#type == "A" && r.name == config.record_name));
        log::info!("A record has been deleted...");
        action_taken = true;
    }

    if remove_aaaa_rec {
        records.retain(|r| !(r.r#type == "AAAA" && r.name == config.record_name));
        log::info!("AAAA record has been deleted...");
        action_taken = true;
    }

    Ok(action_taken)
}
