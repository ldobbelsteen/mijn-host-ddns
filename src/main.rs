#![warn(clippy::pedantic)]

use anyhow::Result;
use clap::Parser;
use ddns::routine;
use reqwest::Client;
use serde::Deserialize;
use std::{fs::read_to_string, time::Duration};

mod ddns;
mod ip;
mod mijnhost;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    domain_name: String,
    api_key: String,
    record_name: String,
    interval: u64,
    manage_records: bool,
}

#[derive(Debug, Parser)]
struct Args {
    #[clap(index = 1, default_value = "./config.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::builder().init();

    let args = Args::parse();
    let file = read_to_string(args.config)?;
    let mut config: Config = toml::from_str(&file)?;

    config.record_name = if config.record_name == "@" {
        config.domain_name.clone() + "."
    } else {
        config.record_name + "." + &config.domain_name
    };

    let client = Client::new();

    if config.interval == 0 {
        routine(&config, &client).await?;
        Ok(())
    } else {
        let mut interval = tokio::time::interval(Duration::from_secs(config.interval));

        loop {
            interval.tick().await;
            routine(&config, &client).await?;
        }
    }
}
