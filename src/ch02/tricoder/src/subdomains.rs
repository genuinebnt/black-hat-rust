use std::collections::HashSet;

use reqwest::blocking::Client;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct CrtShEntry {
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

#[derive(Debug)]
pub struct Subdomain {
    domain: String,
    open_ports: Vec<i32>,
}

impl std::fmt::Display for Subdomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Domain: {}, open_ports: {}",
            self.domain,
            self.open_ports
                .iter()
                .copied()
                .map(|value| value.to_string())
                .collect::<String>()
        )
    }
}

pub struct Subdomains(Vec<Subdomain>);

impl From<Vec<Subdomain>> for Subdomains {
    fn from(value: Vec<Subdomain>) -> Self {
        Subdomains(value)
    }
}

impl std::fmt::Display for Subdomains {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().fold(Ok(()), |result, domain| {
            result.and_then(|_| write!(f, "{}", domain))
        })
    }
}

pub fn enumerate(http_client: &Client, target: &str) -> Result<Subdomains, Error> {
    let entries: Vec<CrtShEntry> = http_client
        .get(format!("https://crt.sh/?q=%25.{}&output=json", target))
        .send()?
        .json()?;

    let mut subdomains: HashSet<String> = entries
        .into_iter()
        .map(|entry| {
            entry
                .name_value
                .split("\n")
                .map(|subdomain| subdomain.trim().to_string())
                .collect::<Vec<String>>()
        })
        .flatten()
        .filter(|subdomain: &String| subdomain != target)
        .filter(|subdomain: &String| !subdomain.contains("*"))
        .collect();
    subdomains.insert(target.into());

    let subdomains: Subdomains = subdomains
        .into_iter()
        .map(|domain| Subdomain {
            domain,
            open_ports: Vec::new(),
        })
        .collect::<Vec<Subdomain>>()
        .into();

    Ok(subdomains)
}
