use serde::Deserialize;

use super::{Ip, Path};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    // The maximum limit for a user with the given id
    // can send maximum request in a single period.
    limit: u32,

    // Maximum count of unique IP addresses
    // that the same user can send requests from
    // in a single period.
    ip_addr_limit: u16,

    // Window time in seconds.
    window_time: u16,

    port: Option<String>,

    host: Option<String>,

    limits: Option<Limits>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Limits {
    path: Option<Path>,
    ip: Option<Ip>,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            path: Some(Path::new(vec![String::new(); 0], 50, 300)),
            ip: Some(Ip::new(vec![String::new(); 0], 50, 300)),
        }
    }
}

impl Limits {
    fn empty() -> Self {
        Self {
            path: None,
            ip: None,
        }
    }
}

impl Config {
    pub fn new(
        limit: Option<u32>,
        ip_addr_limit: Option<u16>,
        window_time: Option<u16>,
        port: Option<String>,
        host: Option<String>,
        limits: Option<Limits>,
    ) -> Self {
        Self {
            limit: limit.unwrap_or(50 as u32),
            ip_addr_limit: ip_addr_limit.unwrap_or(16 as u16),
            window_time: window_time.unwrap_or(300 as u16),
            port: Some(port.unwrap_or("8000".to_owned())),
            host: Some(host.unwrap_or("127.0.0.1".to_owned())),
            limits: Some(limits.unwrap_or(Limits::empty())),
        }
    }

    pub fn limit(&self) -> u32 {
        self.limit
    }

    pub fn set_limit(&mut self, limit: u32) -> u32 {
        self.limit = limit;

        self.limit
    }

    pub fn ip_addr_limit(&self) -> u16 {
        self.ip_addr_limit
    }

    pub fn set_ip_addr_limit(&mut self, limit: u16) -> u16 {
        self.ip_addr_limit = limit;

        self.ip_addr_limit
    }

    pub fn window_time(&self) -> u16 {
        self.window_time
    }

    pub fn set_window_time(&mut self, window_time: u16) -> u16 {
        self.window_time = window_time;

        self.window_time
    }

    pub fn host_and_port(&self) -> String {
        let host_and_port = vec![self.host.clone().unwrap(), self.port.clone().unwrap()];
        host_and_port.join(":").to_owned()
    }

    pub(crate) fn limits_is_some(&self) -> bool {
        self.limits.is_some()
    }

    pub fn limit_path_is_some(&self) -> bool {
        if self.limits_is_some() {
            return self.limits.as_ref().unwrap().path.is_some();
        }

        false
    }

    pub fn limit_ip_is_some(&self) -> bool {
        if self.limits_is_some() {
            return self.limits.as_ref().unwrap().ip.is_some();
        }

        false
    }

    pub fn limited_paths(&self) -> Option<Vec<String>> {
        if self.limit_path_is_some() {
            return self.limits.as_ref().unwrap().path.as_ref().unwrap().paths();
        }

        None
    }

    pub fn path_limit(&self) -> Option<u16> {
        if self.limit_path_is_some() {
            return self.limits.as_ref().unwrap().path.as_ref().unwrap().limit();
        }

        None
    }

    pub fn path_window_time(&self) -> Option<u16> {
        if self.limit_path_is_some() {
            return self
                .limits
                .as_ref()
                .unwrap()
                .path
                .as_ref()
                .unwrap()
                .window_time();
        }

        None
    }

    pub fn limited_ip_addresses(&self) -> Option<Vec<String>> {
        if self.limit_ip_is_some() {
            return self
                .limits
                .as_ref()
                .unwrap()
                .ip
                .as_ref()
                .unwrap()
                .ip_addresses();
        }

        None
    }

    pub fn ip_addresses_limit(&self) -> Option<u16> {
        if self.limit_ip_is_some() {
            return self.limits.as_ref().unwrap().ip.as_ref().unwrap().limit();
        }

        None
    }

    pub fn ip_addresses_window_time(&self) -> Option<u16> {
        if self.limit_ip_is_some() {
            return self
                .limits
                .as_ref()
                .unwrap()
                .ip
                .as_ref()
                .unwrap()
                .window_time();
        }

        None
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            limit: 50 as u32,
            ip_addr_limit: 5 as u16,
            window_time: 300 as u16,
            host: Some("127.0.0.1".to_owned()),
            port: Some("8000".to_owned()),
            limits: Some(Limits::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_updating_window_log() {
        let mut config = Config::default();

        assert!(config.limit() == 50);
        assert!(config.ip_addr_limit() == 5);
        assert!(config.window_time() == 300);

        config.set_window_time(500);
        config.set_limit(100);
        config.set_ip_addr_limit(25);

        assert!(config.window_time() == 500);
        assert!(config.limit() == 100);
        assert!(config.ip_addr_limit() == 25);
    }
}
