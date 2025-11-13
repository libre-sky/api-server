use std::net::{Ipv4Addr, SocketAddr};

#[derive(Debug, Clone)]
pub(crate) struct Config {
    pub(crate) interface: SocketAddr,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            interface: SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080),
        }
    }
}
