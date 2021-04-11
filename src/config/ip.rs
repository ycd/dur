use std::net::Ipv4Addr;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Ip {
    ip_addresses: Option<Vec<Ipv4Addr>>,
    limit: Option<u16>,
    window_time: Option<u16>,
}

impl Ip {
    #[allow(dead_code)]
    pub fn new<I, T>(ip_addrs: I, limit: u16, window_time: u16) -> Self
    where
        T: Into<Ipv4Addr>,
        I: IntoIterator<Item = T>,
    {
        Self {
            ip_addresses: Some(ip_addrs.into_iter().map(Into::into).collect()),
            limit: Some(limit),
            window_time: Some(window_time),
        }
    }

    pub fn ip_addresses(&self) -> Option<Vec<Ipv4Addr>> {
        self.ip_addresses.clone()
    }

    pub fn window_time(&self) -> Option<u16> {
        self.window_time
    }

    pub fn limit(&self) -> Option<u16> {
        self.limit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_new() {
        let ip = Ip::new(
            vec![
                Ipv4Addr::new(127, 0, 0, 1),
                Ipv4Addr::new(154, 10, 94, 111),
                Ipv4Addr::new(10, 51, 144, 201),
            ],
            300,
            400,
        );

        assert_eq!(
            ip.ip_addresses.clone().unwrap()[0],
            Ipv4Addr::new(127, 0, 0, 1)
        );
        assert_eq!(
            ip.ip_addresses.clone().unwrap()[1],
            Ipv4Addr::new(154, 10, 94, 111)
        );
        assert_eq!(
            ip.ip_addresses.clone().unwrap()[2],
            Ipv4Addr::new(10, 51, 144, 201)
        );
        assert_eq!(ip.limit, Some(300));
        assert_eq!(ip.window_time, Some(400));
    }
}
