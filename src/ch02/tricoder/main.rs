use anyhow::Result;
use rayon::prelude::*;
use reqwest::blocking::Client;
use std::fs;
use std::time::Duration;

mod common_ports;
mod subdomain;
mod ports;

use ports::Subdomain;

fn main() -> Result<()> {
    let wordlist: Vec<String> = fs::read_to_string("input/subdomains_wordlist.txt")?
        .split("\n")
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect();

    let target = "google.com";

    let http_client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let pool = rayon::ThreadPoolBuilder::new().num_threads(256).build()?;

    pool.install(|| {
        let valid_subdomains: Vec<String> = wordlist
            .into_par_iter()
            .map(|word| format!("https://{}.{}", word, target))
            .filter(|target| subdomain::enumerate(&http_client, target))
            .collect();

        println!("{:?}", valid_subdomains);
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_wordlist() {
        let wordlist: Vec<String> = fs::read_to_string("input/subdomains_wordlist.txt")
            .unwrap()
            .split("\n")
            .into_iter()
            .map(|value| value.to_string())
            .filter(|value| !value.is_empty())
            .collect();

        println!("{:?}", wordlist);
    }
}
