use rayon::prelude::*;
use reqwest::Client;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::time::Duration;

#[derive(Debug)]
pub struct Subdomain {
    domain: String,
    open_ports: Vec<u16>,
}

pub fn scan_ports(domain: String, ports: &[u16]) -> Subdomain {
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024", domain)
        .to_socket_addrs()
        .unwrap()
        .collect();

    let open_ports: Vec<u16> = ports
        .into_par_iter()
        .filter(|port| scan_port(socket_addresses[0], **port))
        .copied()
        .collect();

    Subdomain {
        domain,
        open_ports: open_ports.to_vec(),
    }
}

fn scan_port(mut address: SocketAddr, port: u16) -> bool {
    let timeout = Duration::from_secs(3);
    address.set_port(port);

    if let Ok(_) = TcpStream::connect_timeout(&address, timeout) {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::ToSocketAddrs;

    use crate::common_ports::MOST_COMMON_PORTS_100;

    #[test]
    fn test_sock_addr() {
        let domain = "yahoo.com";
        let addrs: Vec<SocketAddr> = format!("{}:1024", domain)
            .to_socket_addrs()
            .unwrap()
            .collect();

        println!("{:?}", addrs);
    }
}
