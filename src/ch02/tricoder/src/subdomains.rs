use std::collections::HashSet;
use std::fmt::Display;

use reqwest::blocking::Client;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct Response {
    issuer_ca_id: i64,
    issuer_name: String,
    common_name: String,
    name_value: String,
    id: i64,
    entry_timestamp: String,
    not_before: String,
    not_after: String,
    serial_number: String,
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Domain name: {}", self.common_name)
    }
}

type Subdomaion = String;
type CrtShEntry = String;

pub fn enumerate(http_client: &Client, target: &str) -> Result<(), Error> {
    let entries: Vec<Response> = http_client
        .get(format!("https://crt.sh/?q=%25.{}&output=json", target))
        .send()?
        .json()?;

    entries.into_iter().for_each(|resp| {
        println!("{:}", resp);
    });

    Ok(())
}
