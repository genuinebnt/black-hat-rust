use crate::subdomains::Subdomain;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};

pub fn scan_ports(subdomain: Subdomain) -> Subdomain {
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
        .to_socket_addrs()
        .expect("port scanner: Creating socket address").collect();

    if socket_addresses.len() == 0 {
        return subdomain;
    }
    subdomain
}
