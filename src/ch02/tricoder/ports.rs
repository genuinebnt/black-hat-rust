use reqwest::Client;

pub struct Subdomain {
    domain: String,
    open_ports: Vec<u16>,
}

pub fn scan_ports(http_client: Client, target: String, ports: &[u16]) -> Subdomain {
    unimplemented!()
}
