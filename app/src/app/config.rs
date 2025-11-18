use std::net::{Ipv4Addr, SocketAddr};

use log::Level;

#[derive(Debug, Clone)]
pub(crate) struct Config {
    pub(crate) interface: SocketAddr,
    pub(crate) log_level: Level,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            interface: SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080),
            log_level: Level::Info,
        }
    }
}
