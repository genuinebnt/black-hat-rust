use crate::common_ports::MOST_COMMON_PORTS_100;
use reqwest::blocking::Client;
use reqwest::Error;
use reqwest::StatusCode;

#[derive(Debug)]
pub enum GetError {
    RequestError(Error),
    NotValid(StatusCode),
}

#[derive(Debug)]
pub struct Subdomain {
    domain: String,
    ports: Vec<u16>,
}

pub fn enumerate(http_client: &Client, target: &str) -> Option<String> {
    let resp = http_client.get(target).send();
    println!("{}", target);
    match resp {
        Ok(resp) => {
            if resp.status().is_success() {
                Some(target.to_owned())
            } else {
                println!("{}", resp.status());
                return None;
            }
        }
        Err(e) => {
            println!("{}", e);
            return None;
        }
    }
}
