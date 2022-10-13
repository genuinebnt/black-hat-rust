use anyhow::Error;
use reqwest::blocking::Client;

mod error;
mod subdomains;
mod ports;

fn main() -> Result<(), Error> {
    let client = Client::new();
    let subdomains = subdomains::enumerate(&client, "kerkour.com")?;
    print!("{:}", subdomains);
    Ok(())
}
